/* src/ip.rs */

use crate::error::NetSemError;
use std::net::IpAddr;

/// Classification of an IP address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IpClass {
	/// Loopback address (e.g., 127.0.0.1, ::1).
	Loopback,
	/// Private network address (RFC 1918, IPv6 ULA).
	Private,
	/// Link-local address (169.254.0.0/16, fe80::/10).
	LinkLocal,
	/// Global unicast address.
	Global,
	/// Multicast address (224.0.0.0/4, ff00::/8).
	Multicast,
	/// Unspecified address (0.0.0.0, ::).
	Unspecified,
	/// Broadcast address (255.255.255.255).
	Broadcast,
	/// Documentation address (TEST-NET-1/2/3, IPv6 Doc).
	Documentation,
	/// Shared address space / Carrier-grade NAT (100.64.0.0/10, RFC 6598).
	SharedAddress,
	/// Benchmarking address (198.18.0.0/15, RFC 2544).
	Benchmarking,
}

/// Parses a string into an IP address.
///
/// # Arguments
///
/// * `s` - A string slice containing the IP address.
///
/// # Returns
///
/// * `Ok(IpAddr)` if the string is a valid IP address.
/// * `Err(NetSemError::InvalidIp)` if the format is invalid.
pub fn parse_ip(s: &str) -> Result<IpAddr, NetSemError> {
	s.parse::<IpAddr>()
		.map_err(|_| NetSemError::InvalidIp(s.to_owned()))
}

/// Classifies an IP address into a high-level category.
///
/// Priority: Loopback -> Unspecified -> Multicast -> Broadcast -> LinkLocal -> Documentation -> Private -> Global.
///
/// # Arguments
///
/// * `ip` - The IP address to classify.
#[must_use]
pub fn classify_ip(ip: IpAddr) -> IpClass {
	if ip.is_loopback() {
		return IpClass::Loopback;
	}
	if ip.is_unspecified() {
		return IpClass::Unspecified;
	}
	if ip.is_multicast() {
		return IpClass::Multicast;
	}

	match ip {
		IpAddr::V4(ipv4) => {
			if ipv4.is_broadcast() {
				return IpClass::Broadcast;
			}
			if ipv4.is_link_local() {
				return IpClass::LinkLocal;
			}
			if ipv4.is_documentation() {
				return IpClass::Documentation;
			}
			// Shared address space / CGNAT: 100.64.0.0/10 (RFC 6598)
			{
				let octets = ipv4.octets();
				if octets[0] == 100 && (octets[1] & 0xc0) == 64 {
					return IpClass::SharedAddress;
				}
			}
			// Benchmarking: 198.18.0.0/15 (RFC 2544)
			{
				let octets = ipv4.octets();
				if octets[0] == 198 && (octets[1] & 0xfe) == 18 {
					return IpClass::Benchmarking;
				}
			}
			if ipv4.is_private() {
				return IpClass::Private;
			}
		}
		IpAddr::V6(ipv6) => {
			// Manual Link-Local check for stability: fe80::/10
			// segments[0] & 0xffc0 == 0xfe80
			if (ipv6.segments()[0] & 0xffc0) == 0xfe80 {
				return IpClass::LinkLocal;
			}

			// Documentation: 2001:db8::/32
			// segments[0] == 0x2001 && segments[1] == 0x0db8
			if ipv6.segments()[0] == 0x2001 && ipv6.segments()[1] == 0x0db8 {
				return IpClass::Documentation;
			}

			// Private (ULA): fc00::/7
			// segments[0] & 0xfe00 == 0xfc00
			if (ipv6.segments()[0] & 0xfe00) == 0xfc00 {
				return IpClass::Private;
			}
		}
	}

	IpClass::Global
}

/// Checks if a string is a valid IP address syntax.
///
/// Does NOT perform DNS lookups.
#[must_use]
pub fn is_valid_ip_literal(s: &str) -> bool {
	s.parse::<IpAddr>().is_ok()
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::net::{Ipv4Addr, Ipv6Addr};

	#[test]
	fn test_parse_ip() {
		assert!(parse_ip("127.0.0.1").is_ok());
		assert!(parse_ip("::1").is_ok());
		assert!(parse_ip("invalid").is_err());
	}

	#[test]
	fn test_classify_ip() {
		// Loopback
		assert_eq!(
			classify_ip(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
			IpClass::Loopback
		);
		assert_eq!(
			classify_ip(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1))),
			IpClass::Loopback
		);

		// Unspecified
		assert_eq!(
			classify_ip(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
			IpClass::Unspecified
		);

		// Multicast
		assert_eq!(
			classify_ip(IpAddr::V4(Ipv4Addr::new(224, 0, 0, 1))),
			IpClass::Multicast
		);
		assert_eq!(
			classify_ip(IpAddr::V6(Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 1))),
			IpClass::Multicast
		);

		// Broadcast
		assert_eq!(
			classify_ip(IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255))),
			IpClass::Broadcast
		);

		// LinkLocal
		assert_eq!(
			classify_ip(IpAddr::V4(Ipv4Addr::new(169, 254, 1, 1))),
			IpClass::LinkLocal
		);
		assert_eq!(
			classify_ip(IpAddr::V6(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1))),
			IpClass::LinkLocal
		);

		// Documentation
		assert_eq!(
			classify_ip(IpAddr::V4(Ipv4Addr::new(192, 0, 2, 1))),
			IpClass::Documentation
		);
		assert_eq!(
			classify_ip(IpAddr::V4(Ipv4Addr::new(198, 51, 100, 1))),
			IpClass::Documentation
		);
		assert_eq!(
			classify_ip(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1))),
			IpClass::Documentation
		);
		assert_eq!(
			classify_ip(IpAddr::V6(Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1))),
			IpClass::Documentation
		);

		// Private
		assert_eq!(
			classify_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))),
			IpClass::Private
		);
		assert_eq!(
			classify_ip(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))),
			IpClass::Private
		);
		assert_eq!(
			classify_ip(IpAddr::V6(Ipv6Addr::new(0xfd00, 0, 0, 0, 0, 0, 0, 1))),
			IpClass::Private
		);

		// SharedAddress (CGNAT)
		assert_eq!(
			classify_ip(IpAddr::V4(Ipv4Addr::new(100, 64, 0, 1))),
			IpClass::SharedAddress
		);
		assert_eq!(
			classify_ip(IpAddr::V4(Ipv4Addr::new(100, 127, 255, 255))),
			IpClass::SharedAddress
		);

		// Benchmarking
		assert_eq!(
			classify_ip(IpAddr::V4(Ipv4Addr::new(198, 18, 0, 1))),
			IpClass::Benchmarking
		);
		assert_eq!(
			classify_ip(IpAddr::V4(Ipv4Addr::new(198, 19, 255, 255))),
			IpClass::Benchmarking
		);

		// Global
		assert_eq!(
			classify_ip(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))),
			IpClass::Global
		);
		assert_eq!(
			classify_ip(IpAddr::V6(Ipv6Addr::new(
				0x2606, 0x4700, 0, 0, 0, 0, 0, 0x1111
			))),
			IpClass::Global
		);

		// IPv6 Unspecified
		assert_eq!(
			classify_ip(IpAddr::V6(Ipv6Addr::UNSPECIFIED)),
			IpClass::Unspecified
		);
	}

	#[test]
	fn test_is_valid_ip_literal() {
		assert!(is_valid_ip_literal("127.0.0.1"));
		assert!(is_valid_ip_literal("::1"));
		assert!(is_valid_ip_literal("192.168.0.1"));
		assert!(!is_valid_ip_literal("invalid"));
		assert!(!is_valid_ip_literal("256.0.0.1"));
		assert!(!is_valid_ip_literal(""));
	}
}
