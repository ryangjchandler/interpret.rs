# Your first node

Let's start creating the data structures for your abstract syntax tree. We need to do this now so that we have something to return from our LALRPOP parser.

First we should probably differentiate between the 2 types of "node" in our language - **statements** and **expressions**.

## What is a statement?

In programming, a statement is a block or sequence of code that executes but **does not** return a value.

A good example is a function definition:

```javascript
function hello() {
    // ...
}
```

The above function definition will be executed by our interpreter, but it won't produce a value of any kind. It instead has a side-effect on the execution environment (we'll cover this in a later chapter).

## What is an expression?

An expression is also a block or sequence of code that **does** produce a value. Any time you run `1 + 2`, the programming language will evaluate that code and tell you that the value is `3`. Expressions can be used to create new values and retrieve existing ones.

A lot of programming languages also allow you to use expressions as statements. The value that gets produced by that expression is never used and is discarded by the interpreter.

## The `ast` module

We'll be separating our code into modules from the get-go. This will save us a lot of refactoring work later on and it's just a good practice to follow.

Create an empty `src/ast.rs` file in your project. The `ast` module will store all of the code related to our abstract-syntax tree.

The new module will also need to be registered with our crate. Update your `src/main.rs` with the following code:

```rust,ignore
mod ast;

fn main() {
    println!("Hello, world!");
}
```

With this in place, the `ast` module can be access as `crate::ast` from anywhere inside of our project. Awesome!

## The `Statement` enumeration

Every top-level piece of code in SimpleScript will be a `Statement`. This will allow us to interpret declarations, definitions and also simpler expressions such as function calls.

We'll represent this as an `enum` in Rust, since we'll have multiple types of `Statement` and Rust doesn't have an object-oriented model for inheritance. Rust's `enum` types are also unique as they allow you to created "tagged unions".

If you've programmed in other low-level languages such as C, you've likely come across the `union` keyword before. Rust allows you to define variants on an `enum` that contain and hold their own pieces of data. That can either be in the form of `struct`-like variant or a tuple.

> For more information about Rust's "tagged unions", I recommend reading [this excellent article by Tony Arcieri](https://tonyarcieri.com/a-quick-tour-of-rusts-type-system-part-1-sum-types-a-k-a-tagged-unions).

For clarity and readability, we'll be using the `struct` form for our variants. Let's start by creating the `enum` and adding a variant to store the data for our `LetStatement` from a previous chapter.

```rust,ignore
#[derive(Debug)]
pub enum Statement {
    Let {
        ident: String,
    }
}
```

> The `Debug` derive-macro will be added to the majority of data types in SimpleScript. It will allow us to debug any problems we encounter in the future.

The `Statement` enumeration now has a single `Let` variant. This variant is able to store a `String`, accessible as `ident`. Of course, our `Let` statements won't just be storing the name of the identifier. We'll also need to store the default value for the variable too.

## The `Expression` enumeration

Following the same logic as the `Statement` enumeration, we can define an `Expression` data type to store all of our expressions.

```rust,ignore
#[derive(Debug)]
pub enum Expression {
    StringLiteral { s: String },
}
```

We'll support the simplest of expressions for now. The `StringLiteral` variant is able to hold a `String` on the `s` field of the variant.

### Adding a value to the `Let` variant

With the `Expression` enumeration in place, we can add a new `value` field to `Statement::Let` that will provide the default value for the variable.

```rust,ignore
#[derive(Debug)]
pub enum Statement {
    Let {
        ident: String,
        value: Expression,
    }
}
```

If you look back at a previous chapter, you'll remember that our flowchart didn't always expect an initial value for a variable. If the parser doesn't find a `=` token after the identifier, it just ends the statement.

Most dynamic languages will initialise the value of this variable to `null` or `nil`. This is exactly what SimpleScript will do.

However there are 2 ways to do this:

1. Store an `Expression::Null` variant inside of `value`.
2. Make `value` an `Option` instead.

Since it's more idiomatic and "correct", we'll opt for an `Option` type. If an initial value isn't found by our parser, we can set the `value = None` instead of a `::Null` expression.

```rust,ignore
# #[derive(Debug)]
#pub enum Statement {
    Let {
        ident: String,
        value: Option<Expression>,
    }
#}
```