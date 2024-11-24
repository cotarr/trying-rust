# trying-rust

With a couple friends on IRC, we decided to try some coding in rust.
We are using the online book "The Rust Programming language" online
at https://doc.rust-lang.org/book/title-page.html. I order the paper book.

My first notes are a little detailed, because I am trying to familiarize
myself with the vocabulary used to describe the build process and rust language.

## Setup

- Setup VM with Debian 12
- Installed rust `rustup`

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

- Installed vscode extension "rust-analyzer"
- Created an empty git repository (this repo) and cloned it.
- Created .gitignore with "*target*" to avoid commit binary executable files`
- Begin at Chapter 1 section 1.1

## 1.1 Hello World

- Created folder 1.1-hello-world and copy main.rs from book
- The `fn main() {` defines main function as entry point to program
- Rust style is indent 4 spaces (not tab)
- The `println!` the exclamation point calls a macro (without ! would be function)
- Most rust expressions end in semi-colon `;`
- Compile stand alone binary executable with `rustc main.rs`
- Run with `./main` to produces: "Hello, World!"
- Added "main" (binary) to .gitignore, since not in target folder

## 1.2 Cargo

- Cargo is Rust’s build system and package manager
- New project with `cargo new hello_cargo`
- Rename folder "1.2-hello_cargo" to sort folders in order
- Created Cargo.toml and src folder
- Unless parent folder is a git repository, initialized a new Git repository
- Get help with `cargo new --help`
- toml (Tom’s Obvious, Minimal Language) is cargo configuration
- Cargo has generated a “Hello, world!” program for you,
- Build dev version with `cargo build`
- Run with `./target/debug/hello_cargo` produced "Hello, World!"
- Build and run with `cargo run`
- Check syntax with `cargo check`
- Help with `cargo build --help`
- Build man pages with `cargo help build`
- Build release with `cargo build --release`
