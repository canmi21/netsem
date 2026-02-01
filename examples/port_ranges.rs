/* examples/port_ranges.rs */

use netsem::{PortClass, classify_port};

fn main() {
	let ports = [22, 80, 443, 3000, 8080, 50000, 65535];

	println!("{:<8} | {:<15}", "Port", "IANA Category");
	println!("{:-<26}", "");

	for p in ports {
		let class = classify_port(p);
		let description = match class {
			PortClass::System => "System/Well-known",
			PortClass::User => "User/Registered",
			PortClass::Dynamic => "Dynamic/Private/Ephemeral",
			_ => "Unknown",
		};
		println!(
			"{:<8} | {:<15} ({})",
			p,
			format!("{:?}", class),
			description
		);
	}
}
