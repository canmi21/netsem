/* src/socket.rs */

use crate::error::NetSemError;
use std::net::SocketAddr;

#[cfg(feature = "check")]
use crate::port::{check_bind_tcp, check_bind_udp};

/// Validates a string as a socket address (IP:Port).
///
/// # Arguments
///
/// * `s` - The string to validate.
///
/// # Returns
///
/// * `Ok(SocketAddr)` if valid.
/// * `Err(NetSemError::InvalidSocketAddr)` if parsing fails.
pub fn validate_socket_addr(s: &str) -> Result<SocketAddr, NetSemError> {
	// std::net::SocketAddr parsing handles "[ipv6]:port" and "ipv4:port"
	s.parse::<SocketAddr>()
		.map_err(|_| NetSemError::InvalidSocketAddr(s.to_owned()))
}

/// Checks if the given socket address can be bound (TCP).
///
/// This attempts an actual OS bind (delegating to `check_bind_tcp`).
///
/// # Arguments
///
/// * `addr` - The socket address to check.
#[cfg(feature = "check")]
pub fn can_bind_tcp(addr: &SocketAddr) -> Result<(), NetSemError> {
	check_bind_tcp(addr.ip(), addr.port())
}

/// Checks if the given socket address can be bound (UDP).
///
/// This attempts an actual OS bind (delegating to `check_bind_udp`).
///
/// # Arguments
///
/// * `addr` - The socket address to check.
#[cfg(feature = "check")]
pub fn can_bind_udp(addr: &SocketAddr) -> Result<(), NetSemError> {
	check_bind_udp(addr.ip(), addr.port())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_validate_socket_addr() {
		assert!(validate_socket_addr("127.0.0.1:8080").is_ok());
		assert!(validate_socket_addr("[::1]:80").is_ok());
		assert!(matches!(
			validate_socket_addr("127.0.0.1").unwrap_err(),
			NetSemError::InvalidSocketAddr(_)
		));
		assert!(matches!(
			validate_socket_addr("256.0.0.1:80").unwrap_err(),
			NetSemError::InvalidSocketAddr(_)
		));
	}

	#[test]
	#[cfg(feature = "check")]
	fn test_can_bind_tcp_local() {
		let addr = validate_socket_addr("127.0.0.1:0").unwrap();
		let result = can_bind_tcp(&addr);
		assert!(result.is_ok(), "TCP bind check failed for 127.0.0.1:0");
	}

	#[test]
	#[cfg(feature = "check")]
	fn test_can_bind_udp_local() {
		let addr = validate_socket_addr("127.0.0.1:0").unwrap();
		let result = can_bind_udp(&addr);
		assert!(result.is_ok(), "UDP bind check failed for 127.0.0.1:0");
	}
}
