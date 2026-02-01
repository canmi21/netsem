/* examples/simple.rs */

use netsem::{validate_port, validate_socket_addr};

#[cfg(feature = "check")]
use netsem::check_bind_tcp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let addr_str = "127.0.0.1:8080";
	println!("Parsing address: {}", addr_str);

	let socket_addr = validate_socket_addr(addr_str)?;
	println!("Valid socket address: {}", socket_addr);

	println!("Validating port: {}", socket_addr.port());
	validate_port(socket_addr.port())?;
	println!("Port is valid.");

	#[cfg(feature = "check")]
	{
		println!("Feature 'check' enabled. Attempting to bind (TCP)...");
		match check_bind_tcp(socket_addr.ip(), socket_addr.port()) {
			Ok(_) => println!("Bind check passed! (Port {} is free)", socket_addr.port()),
			Err(e) => println!("Bind check failed: {}", e),
		}
	}

	#[cfg(not(feature = "check"))]
	{
		println!("Feature 'check' is disabled. Skipping OS-level bind check.");
	}

	Ok(())
}
