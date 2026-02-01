/* src/port.rs */

use crate::error::NetSemError;

#[cfg(feature = "check")]
use std::net::{IpAddr, SocketAddr};
#[cfg(feature = "check")]
use std::time::Duration;

/// Classification of a port number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PortClass {
	/// System ports (0-1023).
	System,
	/// User ports (1024-49151).
	User,
	/// Dynamic/Private ports (49152-65535).
	Dynamic,
}

/// Validates a port number, disallowing port 0.
///
/// # Returns
///
/// * `Ok(())` if `p` is in range 1..=65535.
/// * `Err(NetSemError::InvalidPort(0))` if `p` is 0.
pub fn validate_port(p: u16) -> Result<(), NetSemError> {
	if p == 0 {
		return Err(NetSemError::InvalidPort(0));
	}
	Ok(())
}

/// Validates a port number, allowing port 0 (wildcard/ephemeral).
///
/// This always returns `Ok(())` as all u16 values are valid in this context.
pub fn validate_port_or_zero(p: u16) -> Result<(), NetSemError> {
	let _ = p;
	Ok(())
}

/// Classifies a port number into its IANA range.
#[must_use]
pub fn classify_port(p: u16) -> PortClass {
	if p < 1024 {
		PortClass::System
	} else if p < 49152 {
		PortClass::User
	} else {
		PortClass::Dynamic
	}
}

/// Internal helper to check binding for TCP or UDP.
#[cfg(feature = "check")]
fn check_bind_inner(ip: IpAddr, port: u16, socket_type: socket2::Type) -> Result<(), NetSemError> {
	use socket2::{Domain, Socket};

	let addr = SocketAddr::new(ip, port);
	let domain = match ip {
		IpAddr::V4(_) => Domain::IPV4,
		IpAddr::V6(_) => Domain::IPV6,
	};

	let socket = Socket::new(domain, socket_type, None).map_err(|e| NetSemError::BindFailed {
		addr: addr.to_string(),
		source: e,
	})?;

	socket
		.set_reuse_address(true)
		.map_err(|e| NetSemError::BindFailed {
			addr: addr.to_string(),
			source: e,
		})?;

	socket
		.bind(&addr.into())
		.map_err(|e| NetSemError::BindFailed {
			addr: addr.to_string(),
			source: e,
		})?;

	Ok(())
}

/// Checks if a TCP socket can bind to the specified IP and port.
#[cfg(feature = "check")]
pub fn check_bind_tcp(ip: IpAddr, port: u16) -> Result<(), NetSemError> {
	check_bind_inner(ip, port, socket2::Type::STREAM)
}

/// Checks if a UDP socket can bind to the specified IP and port.
#[cfg(feature = "check")]
pub fn check_bind_udp(ip: IpAddr, port: u16) -> Result<(), NetSemError> {
	check_bind_inner(ip, port, socket2::Type::DGRAM)
}

/// Checks if a TCP connection can be established to the specified IP and port.
#[cfg(feature = "check")]
pub fn check_connect_tcp(
	ip: IpAddr,
	port: u16,
	timeout: Option<Duration>,
) -> Result<(), NetSemError> {
	use socket2::{Domain, Socket, Type};

	let addr = SocketAddr::new(ip, port);
	let domain = match ip {
		IpAddr::V4(_) => Domain::IPV4,
		IpAddr::V6(_) => Domain::IPV6,
	};

	let socket = Socket::new(domain, Type::STREAM, None).map_err(|e| NetSemError::ConnectFailed {
		addr: addr.to_string(),
		source: e,
	})?;

	let addr_sock = addr.into();

	if let Some(duration) = timeout {
		socket
			.connect_timeout(&addr_sock, duration)
			.map_err(|e| NetSemError::ConnectFailed {
				addr: addr.to_string(),
				source: e,
			})?;
	} else {
		socket
			.connect(&addr_sock)
			.map_err(|e| NetSemError::ConnectFailed {
				addr: addr.to_string(),
				source: e,
			})?;
	}

	Ok(())
}

/// Checks if a UDP socket can "connect" to the specified IP and port.
///
/// **Note:** For UDP, `connect` merely sets the default destination address.
/// It does NOT perform a handshake or verify network reachability in the same way TCP does.
#[cfg(feature = "check")]
pub fn check_connect_udp(ip: IpAddr, port: u16) -> Result<(), NetSemError> {
	use socket2::{Domain, Socket, Type};

	let addr = SocketAddr::new(ip, port);
	let domain = match ip {
		IpAddr::V4(_) => Domain::IPV4,
		IpAddr::V6(_) => Domain::IPV6,
	};

	let socket = Socket::new(domain, Type::DGRAM, None).map_err(|e| NetSemError::ConnectFailed {
		addr: addr.to_string(),
		source: e,
	})?;

	// UDP connect just sets the default remote address
	socket
		.connect(&addr.into())
		.map_err(|e| NetSemError::ConnectFailed {
			addr: addr.to_string(),
			source: e,
		})?;

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_classify_port() {
		assert_eq!(classify_port(0), PortClass::System);
		assert_eq!(classify_port(1023), PortClass::System);
		assert_eq!(classify_port(1024), PortClass::User);
		assert_eq!(classify_port(49151), PortClass::User);
		assert_eq!(classify_port(49152), PortClass::Dynamic);
		assert_eq!(classify_port(65535), PortClass::Dynamic);
	}

	#[test]
	fn test_validate_port() {
		assert!(validate_port(0).is_err());
		assert!(validate_port(80).is_ok());
		assert!(validate_port(65535).is_ok());
	}

	#[test]
	fn test_validate_port_or_zero() {
		assert!(validate_port_or_zero(0).is_ok());
		assert!(validate_port_or_zero(80).is_ok());
	}

	#[test]
	#[cfg(feature = "check")]
	fn test_check_bind_ephemeral() {
		use std::net::{IpAddr, Ipv4Addr};
		let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
		// Binding to port 0 should generally succeed as the OS assigns a port.
		let result = check_bind_tcp(ip, 0);
		assert!(
			result.is_ok(),
			"Failed to bind tcp to ephemeral port: {:?}",
			result.err()
		);

		let result_udp = check_bind_udp(ip, 0);
		assert!(
			result_udp.is_ok(),
			"Failed to bind udp to ephemeral port: {:?}",
			result_udp.err()
		);
	}
}
