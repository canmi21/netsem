/* src/lib.rs */

#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! # netsem
//!
//! `netsem` (Network Semantics) provides pure, synchronous utilities for parsing,
//! classifying, and validating IP addresses, ports, and socket addresses.
//!
//! It offers a strict separation between logic/validation and OS-level operations.
//! OS-level checks (binding, connecting) are available only via the `check` feature.

/// Error types and handling.
pub mod error;
/// IP address parsing and classification.
pub mod ip;
/// Port validation, classification, and OS-level checks.
pub mod port;
/// Socket address validation and binding checks.
pub mod socket;

// Re-export core types for convenience
pub use error::NetSemError;
pub use ip::{IpClass, classify_ip, is_valid_ip_literal, parse_ip};
pub use port::{PortClass, classify_port, validate_port, validate_port_or_zero};

#[cfg(feature = "check")]
pub use port::{check_bind_tcp, check_bind_udp, check_connect_tcp};

pub use socket::validate_socket_addr;

#[cfg(feature = "check")]
pub use socket::{can_bind_tcp, can_bind_udp};
