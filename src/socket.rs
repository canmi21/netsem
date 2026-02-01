/* src/socket.rs */

use crate::error::NetSemError;
use std::net::SocketAddr;

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
///
/// # Examples
///
/// ```
/// use netsem::validate_socket_addr;
///
/// let addr = validate_socket_addr("127.0.0.1:8080").unwrap();
/// assert_eq!(addr.port(), 8080);
///
/// assert!(validate_socket_addr("not-an-address").is_err());
/// ```
pub fn validate_socket_addr(s: &str) -> Result<SocketAddr, NetSemError> {
	// std::net::SocketAddr parsing handles "[ipv6]:port" and "ipv4:port"
	s.parse::<SocketAddr>()
		.map_err(|_| NetSemError::InvalidSocketAddr(s.to_owned()))
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
}
