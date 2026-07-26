# rust-up

Learning Rust Programming the old school way!

## Notes

### Basics

#### Build System

- `rustc`: The compiler
- [`cargo`](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html): build and the package manager
  - `cargo new <project name>`
  - `cargo init` -> if project directory already exists
  - `Cargo.toml` -> config file
  - `Cargo.lock` -> created and maintained by `cargo`
  - `cargo build` -> creates an executable file in `target/debug/`, because the default build is a `debug` build
  - To run the program `./target/debug/<executable>`
  - Running `cargo buil`d for the first time causes `Cargo` to create a new file at the top level: `Cargo.lock`.
    - This file keeps track of the exact versions of dependencies in your project
  - Use `cargo run` to compile the code and then run the resultant executable all in one command
  - `cargo check` command checks your code to make sure it compiles but doesn’t produce an executable
  - `cargo` expects your source files to live inside the `src` directory
    - The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code.
    - Using `cargo` helps you organize your projects. There’s a place for everything, and everything is in its place.
- In Rust packages of code are referred to as `crates`
- Adding package as project dependency: `cargo add <package name>`
- Installing packages: `cargo install <package name>`

#### Terminologies and Conventions

- `macro`: macro is like a function, but with an exclamation mark (!) after it. eg: `println!()` is a macro.
  - similar to functions (they execute things), but they do not always follow the same rules as functions.
- `comments`: // single line, /* */ multiline

#### Variables

- use the `let` keyword and specify the name of the variable
  - eg: `let name = "Bob";`
  - println!("My first name is: {}", name);
  - uses `{}` as a placeholder in println!() to show variable values.
  - type of a variable is decided by the value you give it. Rust looks at the value and automatically chooses the right type.
  - You can also specify the type eg: `let my_num: i32 = 5;`
    - Basic data types in Rust are divided into different groups:
      - Numbers - Whole numbers and decimal numbers (`i32`, `f64`)
      - Characters - Single letters or symbols (`char`)
      - Strings - Text, a sequence of characters (`&str`)
      - Booleans - True or false values (`bool`)
- By default, variables in Rust cannot be changed after they are created:
  - `let x = 5;`
  - `x = 10; // Error`
  - If you want to change the value of a variable, you must use the mut keyword (which means mutable/changeable):
    - `let mut x = 5;`
    - `x = 10;`

#### Constants

- You must write the type when creating a constant. You cannot let Rust guess the type like you can with regular variables:
  - `const BIRTHYEAR: i32 = 1980;`

#### String Formatting Placeholders

The most common use of {} is inside macros like `println!`, `format!`, or `panic!`. It acts as a positional placeholder that gets replaced by the value of a variable.

- Basic usage: `println!("Hello, {}!", "world");`
- With named arguments: `println!("{name}", name="Alice");`
- `{:?}` is the format specifier for the Debug trait, which allows you to print data in a format intended for developers and debugging

## Examples Folder

- `examples` directory is a standard convention recognized by `Cargo` to store standalone executable files that showcase how to use your library or package
  - The most common implementation is a standard `.rs` file containing a main function inside the `examples/` folder
  - If an example gets too large, you can create a subfolder inside `examples/.` Cargo will automatically seek a `main.rs` file inside that subfolder to compile it as the entry point.
  - To run a specific example: `cargo run --example <example folder name>`

## References

- [The Rust Programming Language Book](https://doc.rust-lang.org/stable/book/)
- [W3Schools/Rust](https://www.w3schools.com/rust)
