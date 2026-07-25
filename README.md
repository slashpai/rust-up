# rust-up

Learning Rust Programming the old school way!

## Notes

### Basics

- `rustc`: The compiler
- [`cargo`](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html): build and the package manager
  - `cargo new <project name>`
  - `cargo init` -> if project directory already exists
  - `Cargo.toml` -> config files
  - `cargo build` -> creates an executable file in `target/debug/`,  because the default build is a `debug` build
  - To run the program `./target/debug/<executable>`
  - Running `cargo buil`d for the first time causes `Cargo` to create a new file at the top level: `Cargo.lock`.
    - This file keeps track of the exact versions of dependencies in your project
  - Use `cargo run` to compile the code and then run the resultant executable all in one command
  - `cargo check` command checks your code to make sure it compiles but doesn’t produce an executable
  - `cargo` expects your source files to live inside the `src` directory
    - The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code.
    - Using `cargo` helps you organize your projects. There’s a place for everything, and everything is in its place.
- In Rust packages of code are referred to as `crates`

## References

- [The Rust Programming Language Book](https://doc.rust-lang.org/stable/book/)
