# erebor

## projects

* [code from book](https://github.com/km3ll/rust/tree/project/code-from-book)
* [guessing game](https://github.com/km3ll/rust/tree/project/guessing-game)

## commands

#### cargo

```bash
// compile and run
cargo run
cargo check

cargo doc --open 
 └─ Build documentation provided by all of your dependencies locally and open it in your browser

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

- [crates.io](https://crates.io/)
- [Naming conventions: snake_case and kebab-case](https://www.theserverside.com/definition/Snake-case)
- [Semantic Versioning](https://semver.org/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)
- [The Cargo Book](https://doc.rust-lang.org/cargo/index.html)
- [TOML - Tom's Obvious Minimal Language (Cargo's config format)](https://toml.io/en/)
