# Netsem

Standardized, pure-functional IP address validation and port checking utilities.

`netsem` (Network Semantics) provides synchronous tools for parsing, classifying, and validating network primitives (IPs, Ports, Sockets). It is designed to be a lightweight foundation for network applications, offering a clear separation between semantic validation (syntax, logic) and OS-level interactions (binding, connecting).

## Features

- **Pure Validation**: Parse and validate IPs and Ports without touching the OS.
- **IP Classification**: Categorize IPs into `Loopback`, `Private`, `Global`, `Multicast`, or `Unspecified`.
- **Port Classification**: Identify `System`, `User`, or `Dynamic` ports.
- **OS Checks (Optional)**: Perform actual `bind` or `connect` checks using the `check` feature (powered by `socket2`).
- **Sync-First**: Zero async dependencies. Ready to be wrapped in `spawn_blocking` if needed.
- **Error Handling**: Uses `thiserror` for structured, inspectable errors.

## Usage Examples

Check the `examples` directory for runnable code:

- **Basic Usage**: [`examples/simple.rs`](examples/simple.rs) - Parse a socket address and optionally check if it can be bound.
- **IP Classification**: [`examples/ip_classification.rs`](examples/ip_classification.rs) - Categorize different IP addresses (Loopback, Private, Global, etc.).
- **Port Ranges**: [`examples/port_ranges.rs`](examples/port_ranges.rs) - Classify ports into System, User, and Dynamic ranges.
- **OS Checks**: [`examples/os_check.rs`](examples/os_check.rs) - Perform actual bind and connect tests (requires `check` feature).

## Installation

```toml
[dependencies]
netsem = { version = "0.1", features = ["full"] }
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `check` | Enables OS-level checks (`check_bind`, `check_connect`) using `socket2`. |
| `serde` | Enables `serde::Serialize` / `serde::Deserialize` on public enums. |
| `full` | Enables `check` + `serde`. |

## License

Released under the MIT License © 2026 [Canmi](https://github.com/canmi21)