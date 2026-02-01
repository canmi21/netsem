/* src/error.rs */

use thiserror::Error;

/// Errors that can occur within the netsem crate.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum NetSemError {
	/// The provided IP address string format is invalid.
	#[error("Invalid IP address format: {0}")]
	InvalidIp(String),

	/// The provided socket address string format is invalid.
	#[error("Invalid socket address format: {0}")]
	InvalidSocketAddr(String),

	/// The provided port is invalid.
	#[error("Invalid port: {0}")]
	InvalidPort(u16),

	/// Failed to bind to the specified address.
	#[error("Failed to bind to {addr}: {source}")]
	BindFailed {
		/// The address string attempted to bind to.
		addr: String,
		/// The underlying IO error.
		source: std::io::Error,
	},

	/// Failed to connect to the specified address.
	#[error("Failed to connect to {addr}: {source}")]
	ConnectFailed {
		/// The address string attempted to connect to.
		addr: String,
		/// The underlying IO error.
		source: std::io::Error,
	},
}
