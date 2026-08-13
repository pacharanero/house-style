# Minimal Rust CLI Shape

Copy this shape when creating a new Rust CLI. `main.rs` is only startup and error handling; `lib.rs` owns the clap surface and behaviour so another binary or host app can call `run()` directly.

- [`main.rs`](main.rs) - thin process wrapper, including SIGPIPE reset on Unix.
- [`lib.rs`](lib.rs) - reusable CLI library with `Cli`, `Commands`, and `run()`.
