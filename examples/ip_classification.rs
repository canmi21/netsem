/* examples/ip_classification.rs */

use netsem::{IpClass, classify_ip, parse_ip};

fn main() {
	let addresses = [
		"127.0.0.1",
		"192.168.1.1",
		"8.8.8.8",
		"224.0.0.1",
		"::1",
		"fe80::1",
		"0.0.0.0",
		"255.255.255.255",
		"192.0.2.1",
	];

	for s in addresses {
		match parse_ip(s) {
			Ok(ip) => {
				let class = classify_ip(ip);
				println!("IP: {s:<15} | Class: {class:?}");

				// Example of semantic matching
				match class {
					IpClass::Loopback => println!("  -> This is a local loopback address."),
					IpClass::Private => println!("  -> This is a private network address."),
					IpClass::LinkLocal => println!("  -> This is a link-local address."),
					IpClass::Global => println!("  -> This is a globally routable address."),
					IpClass::Multicast => println!("  -> This is a multicast address."),
					IpClass::Broadcast => println!("  -> This is a broadcast address."),
					IpClass::Documentation => println!("  -> This is a documentation address."),
					IpClass::Unspecified => println!("  -> This is an unspecified address."),
					IpClass::SharedAddress => println!("  -> This is a shared/CGNAT address."),
					IpClass::Benchmarking => println!("  -> This is a benchmarking address."),
					_ => println!("  -> Unknown classification."),
				}
			}
			Err(e) => eprintln!("Failed to parse {s}: {e}"),
		}
	}
}
