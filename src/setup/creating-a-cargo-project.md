# Creating a Cargo project

To start working on our interpreter, we need to do some setup. Let's create a Rust project and make sure we can build an executable binary.

## Prerequisites

Before continuing, you'll need to have Rust installed on your machine. I'm going to cover the steps here, since they've been well documented elsewhere.

If you haven't got Cargo and Rust installed, I recommend reading the [Install Rust](https://www.rust-lang.org/tools/install) page on the official Rust website.

## Creating the project

Open a terminal window and create a new Cargo project.

```bash
cargo new simplescript
```

This will create a directory named `simplescript`. Enter the directory.

```bash
cd simplescript
```

To double check that everything is setup correctly, run the following command in the project directory:

```bash
cargo run
```

With any luck, you'll see `Hello, world!` printed in your terminal window.
