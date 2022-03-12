# Installing LALRPOP

As mentioned in ["What is a parser?"](../parsing/what-is-a-parser.md), we'll start off by using [LALRPOP](https://github.com/lalrpop/lalrpop) to generate a parser for our language.

LALRPOP is an `LR(1)` parser generator for Rust. The term `LR(1)` simply means that the parser will only ever lookahead for a single terminal / pattern.

## Dependencies

To get started, you'll need to add the following dependencies to your project's Cargo.toml file.

```toml
[build-dependencies]
lalrpop = "0.19.7"

[dependencies]
lalrpop-util = "0.19.7"
regex = "1"
```

> If you've already made a start and added other dependencies, be sure to merge them correctly.

## Build scripts

Cargo provides a complete build system for your Rust projects. One of the main features is the ability to execute build scripts before the compilation of your project.

LALRPOP uses a build script to process your grammar file and output valid Rust code for use in our interpreter.

You can create a build script by placing a `build.rs` file in the **root** of the project.

```rust,ignore
extern crate lalrpop;

fn main() {
    lalrpop::process_root().unwrap();
}
```

The `extern crate` declaration instructs Rust to import the `lalrpop` module defined in our `[build-dependencies]`.

LALRPOP then provides a helper function that processes a grammar file in our project. The result of this is unwrapped to ensure any errors bubble up and cause the build script to fail.

## The grammar file

Right now, LALRPOP doesn't have anything to process. Let's fix that by creating an empty `src/parser.lalrpop` file in your project.

With an empty file in place, you can run your project (`cargo run`) and see an error in the console.

```
Caused by:
  process didn't exit successfully: `/Users/ryan/path/target/debug/build/book-d2c25b48ff14d1e7/build-script-build` (exit status: 1)
  --- stdout
  processing file `/Users/ryan/path/src/parser.lalrpop`
  /Users/ryan/path/src/parser.lalrpop:1:1: 1:0 error: unexpected end of file
```

We can see that LALRPOP is trying to process our grammar file but it doesn't actually have anything to process. Good stuff!