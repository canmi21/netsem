# Sigterm

Signal-aware async control and cancellation primitives for Tokio.

`sigterm` abstracts away the boilerplate of listening for system signals (`Ctrl+C`, `SIGTERM`, etc.) and coordinating shutdown across multiple asynchronous tasks.

## Features

- **Signal Waiting**: Wait for `Ctrl+C` or `SIGTERM` across platforms with a single `await`. Use `try_wait()` for non-panicking version.
- **Cancellation Tokens**: Hierarchy-based cancellation (parent cancels child) powered by `tokio-util`.
- **Shutdown Primitives**:
  - `Shutdown`: One-shot channel for single-task termination.
  - `Broadcast`: Notify multiple subscribers of a shutdown event.
  - `ShutdownGuard`: RAII guard that triggers shutdown when dropped (useful for panics).
- **Framework Integration**: `shutdown_signal()` helper designed for seamless integration with `axum::serve`.
- **Unix Extensions**: Listen for custom signal sets (`SIGHUP`, `SIGQUIT`, etc.) on Unix systems.

## Usage Examples

Check the `examples` directory for runnable code:

- **Basic Usage**: [`examples/simple.rs`](examples/simple.rs) - Wait for a simple shutdown signal.
- **Server Integration**: [`examples/shutdown_signal.rs`](examples/shutdown_signal.rs) - Combine system signals with internal cancellation (e.g., for Axum).
- **Task Orchestration**: [`examples/broadcast.rs`](examples/broadcast.rs) - Coordinate multiple workers.
- **Hierarchical Cancellation**: [`examples/cancellation.rs`](examples/cancellation.rs) - Manage tree-structured tasks.
- **Scope Guard**: [`examples/guard.rs`](examples/guard.rs) - Ensure shutdown on exit or panic.

## Installation

```toml
[dependencies]
sigterm = { version = "0.3", features = ["full"] }
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `signal` | Enables signal handling (Ctrl+C, SIGTERM) - enabled by default. |
| `sync` | Enables synchronization primitives (`Shutdown`, `Broadcast`). |
| `macros` | Enables Tokio macro support. |
| `rt` | Enables Tokio runtime support (required for `wait_for`). |
| `cancel` | Enables hierarchical cancellation tokens via `tokio-util`. |
| `time` | Enables timeout support for signal waiting. |
| `tracing` | Enables optional tracing instrumentation for debugging. |
| `full` | Enables all features above. |

## License

Released under the MIT License © 2026 [Canmi](https://github.com/canmi21)
