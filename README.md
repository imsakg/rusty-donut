# rusty-boilerplate
---
An boilerplate for personal Rust projects.

## Usage

That boilerplate comes with simple directory parser. Do whatever you want on on it.

```bash
cargo run --release main src/main.rs -- main src/main.rs
```

or you can use cargo watch for auto-rebuild

```bash
cargo watch -q -c -w src -x 'run --release -- main src/main.rs'
```

## Dependencies

* **[Anyhow](https://crates.io/crates/anyhow)**
This library provides anyhow::Error, a trait object based error type for easy idiomatic error handling in Rust applications.

* **[Clap](https://crates.io/crates/clap)**
Create your command-line parser, with all of the bells and whistles, declaratively or procedurally.

* **[Thiserror](https://crates.io/crates/thiserror)**
This library provides a convenient derive macro for the standard library's std::error::Error trait.

* **[Tokio](https://crates.io/crates/tokio)**
A runtime for writing reliable, asynchronous, and slim applications with the Rust programming language.