/* examples/os_check.rs */

//!
//! Requires the `check` feature: `cargo run --example os_check --features check`

#[cfg(feature = "check")]
use netsem::port::check_connect_udp;
use netsem::{check_bind_tcp, check_bind_udp, check_connect_tcp};
#[cfg(feature = "check")]
use std::time::Duration;

fn main() {
	#[cfg(not(feature = "check"))]
	{
		println!("This example requires the 'check' feature to perform OS operations.");
		println!("Please run with: cargo run --example os_check --features check");
	}

	#[cfg(feature = "check")]
	{
		let localhost = "127.0.0.1".parse().unwrap();

		// 1. Check if we can bind to a port (Port 0 lets OS pick a free one)
		println!("Checking if we can bind TCP to 127.0.0.1:0...");
		match check_bind_tcp(localhost, 0) {
			Ok(_) => println!("Successfully verified TCP bind capability on an ephemeral port."),
			Err(e) => eprintln!("TCP Bind check failed: {e}"),
		}

		println!("Checking if we can bind UDP to 127.0.0.1:0...");
		match check_bind_udp(localhost, 0) {
			Ok(_) => println!("Successfully verified UDP bind capability on an ephemeral port."),
			Err(e) => eprintln!("UDP Bind check failed: {e}"),
		}

		// 2. Check if something is listening on a port (e.g., DNS on 53)
		let dns_ip = "8.8.8.8".parse().unwrap();
		println!("\nChecking TCP connection to Google DNS (8.8.8.8:53) with 2s timeout...");
		match check_connect_tcp(dns_ip, 53, Some(Duration::from_secs(2))) {
			Ok(_) => println!("Successfully connected to 8.8.8.8:53."),
			Err(e) => eprintln!("TCP Connection check failed (this is normal if offline): {e}"),
		}

		println!("\nChecking UDP 'connection' to Google DNS (8.8.8.8:53)...");
		// Note: UDP connect is just setting default address, not a handshake.
		match check_connect_udp(dns_ip, 53) {
			Ok(_) => println!("Successfully set default UDP destination to 8.8.8.8:53."),
			Err(e) => eprintln!("UDP Connect check failed: {e}"),
		}
	}
}
