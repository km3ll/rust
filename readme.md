# rust

## commands

#### cargo

```bash
// compile and run
cargo run
cargo check

// runable file
cargo build
 └─ ./target/debug/erebor
cargo build --release
 └─ ./target/release/erebor

// new project
cargo new erebor
```

#### compiler

```bash
rustc --version
rustc src/main.rs --out-dir target
 └─ ./target/main
```

#### rustup

```bash
rustup update
rustup doc
rustfmt src/main.rs
```

## references

- [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)
- [The Cargo Book](https://doc.rust-lang.org/cargo/index.html)
- [TOML - Tom's Obvious Minimal Language (Cargo's config format)](https://toml.io/en/)
- [Naming conventions: snake_case and kebab-case](https://www.theserverside.com/definition/Snake-case)