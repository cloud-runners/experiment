# Experiment

![Rust](https://img.shields.io/badge/rust-2024_edition-orange?logo=rust)
![axum](https://img.shields.io/badge/axum-0.8-blue)
![tokio](https://img.shields.io/badge/tokio-1-blue)

A minimal HTTP server written in Rust using [axum](https://github.com/tokio-rs/axum) and [tokio](https://tokio.rs/). Returns `Hello, World!` at the root route.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

## Running

```bash
cargo run
```

The server listens on `http://0.0.0.0:3000`.

## Testing

```bash
cargo test
```
