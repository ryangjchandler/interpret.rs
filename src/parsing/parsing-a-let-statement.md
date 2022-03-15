# Parsing a "let" statement

> **Warning**: This chapter is quite long and covers a lot of LALRPOP-specific stuff. There's absolutely no harm in reading things multiple times!

Now that we have a `Statement` and `Expression` type, we can start telling LALRPOP how to parse our `let` statements.

To do this, we need to provide LALRPOP with a syntactical representation of the statement. This involves adding some grammar rules to the `parser.lalrpop` file and including some Rust code.

LALRPOP needs to know where all of our data types are coming from. Luckily, we can mix a little bit of Rust code in with our grammar. Just like we would in Rust, we need to import our `ast` module into the LALRPOP file.

```rust,ignore
use crate::ast::*;

grammar;
```

We'll do a wildcard import so that we can reference our data-types with their basename, instead of prefixing them with `ast::`.

With the `ast` module imported, we need to let LALRPOP know what structures should be exposed. We do this by defining a list of patterns that needs to be matched in our source code.

The parser will produce a series of `Statement` variants that represent our `Program`. To keep the terminology consistent, we can say that LALRPOP needs to return a `Program` or `Vec<Statement>` (vector of `Statement` variants).

```rust,ignore
pub Program: Vec<Statement> = {

}
```

LALRPOP's grammar looks a lot like Rust's. This is a good thing since we'll be embedding Rust code inside of the grammar file too, just like we did for the `use` statement.

The code above is telling LALRPOP to expose a parser that returns a `Vec<Statement>`. When LALRPOP generates it's own Rust code for us to use, it will generate a `ProgramParser` structure that accepts a `&str` of source code and returns our defined output type.

If you try to `cargo run` your project now, the LALRPOP error will disappear and you'll see the default `Hello, world!` message. Yay for progress!

## Defining a pattern

Right now, LALRPOP doesn't _actually_ return anything. It knows how to generate the code, but since we're not interacting with the parser yet our program will compile and execute without any errors.

Each LALRPOP declaration (grammar rule) is assigned a map of `pattern => result` pairs. You can think of it like a `match` expression. On the left-hand side of the `=>`, we'll define the pattern that we're looking for and on the right-hand side, we'll return a value.

To do this though, we need to tell LALRPOP what a `Statement` is. We can add the following code after the `Program` block.

```rust,ignore
Statement: Statement = {

}
```

The code here is telling LALRPOP to create a new private set of patterns called `Statement`. This set of patterns is always expected to return one of our `ast::Statement` variants. We'll be able to reference this set of patterns from elsewhere in our LALRPOP grammar, mainly the `Program` block.

The reason we have omitted the `pub` at the beginning of the definition is because we won't need to parse individual `Statement` patterns from our program. LALRPOP is handling that for us by returning `Vec` of them from `Program`.

To make LALRPOP return a vector of `Statement` variants from `Program`, we need to give it a `pattern => result` expression. Back in our `Program` block, we can add the following code:

```rust
pub Program: Vec<Statement> = {
    <statements:Statement*> => statements,
}
```

Let's break it down again. The left-hand side is the pattern we're looking for and the right-hand side is the result of that pattern.

The `<statements:Statement*>` pattern is very similar to a capture-group in a regular expression. Each of these capture groups can be given a name and a sub-pattern to match against. The name will be `statements` and the pattern we're looking for is `Statement*`. A more generalised description would be `<name:pattern>`.

The pattern here references the `Statement` definition we setup a minute ago. The trailing `*` (asterisk) is referred to as a "macro" in LALRPOP. It instructs the parser to look for **0 or more** patterns that match the `Statement` definition. LALRPOP knows that it's looking for (potentially) multiple matches, so it will wrap them up into a `Vec` for us.

Let's go back and update the `Statement` definition to actually match against something, starting with the simplest `let` statement possible:

```rust,ignore
Statement: Statement = {
    "let" <name:Identifier> => Statement::Let { name: name, value: None }
}
```

We've introduced a new concept here and that is matching against literal strings. LALRPOP comes with it's own tokenizer / lexer which means we don't need to hand write all of our token logic. Instead, we can tell LALRPOP to look for certain strings but not to capture them. This is really helpful when starting on a new programming language since you don't need to worry about tokens at all. You can just provide a string and carry on.

You might have noticed the `Identifier` type here too. Before we carry on, let's also add this definition to our grammar file.

```rust,ignore
Identifier: String = {
    <i:r"[a-zA-Z_]+"> => i.to_string(),
}
```

Instead of specifying a grammar definition in the capture group, we're using a regular-expression string instead. We don't need to store this anywhere since we won't be using it again, we'll be using the new `Identifier` definition instead.

The right-hand side is literal Rust code. The capture group will create a variable using the name we provide (`i`) and we can interact with it like we would anywhere else in our Rust code.

The capture group will extract a `&str` (string slice) from our source code. We want a `String` so we can move it around without lifetimes, hence the `.to_string()` call.

Looking back up at `Statement`, we now know that the `name` capture group will store a `String` inside of the variable `name`.

```rust,ignore
Statement: Statement = {
    "let" <name:Identifier> => Statement::Let { name: name, value: None }
}
```

We'll save the `value` parsing for now, that'll be in the next chapter. 

The right-hand side of our pseudo match clause shows some more Rust code. We're parsing out a `let` statement so we need to construct a `Let` variant.

Again, it's all just Rust code. We know we have a `name` variable so we can assign that to the `name` field of the variant. We're ignoring the value for now, so we'll assign `None`.

Both our field and variable have the same name, so we can get even fancier and allow Rust to implictly do the assignment for us by removing the `: name` part.

```rust,ignore
Statement: Statement = {
    "let" <name:Identifier> => Statement::Let { name, value: None }
}
```

That's all it takes to parse out the simplest `let` statement in SimpleScript. Well, not quite...

We haven't actually tried parsing anything yet. Let's update the `src/main.rs` file to parse out a static string (for now).

```rust,ignore
mod ast;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(parser);

fn main() {
    let ast = parser::ProgramParser::new()
        .parse("let foo")
        .unwrap();

    dbg!(ast);
}
```

There's quite a bit of new logic here. Let's go through it bit-by-bit:

```rust,ignore
#[macro_use]
extern crate lalrpop_util;
```

By defaut Rust macros are scoped to the crate / module they're defined in. By decorating the import with `#[macro_use]`, we're instructing Rust to take the macros from the module and make them available to the current context.

```rust,ignore
lalrpop_mod!(parser);
```

The `#[macro_use]` then exposes a new `lalrpop_mod!()` macro. This macro handles all of the magic of creating a Rust module at compile-time. When LALRPOP processes our grammar file, this macro takes the generated Rust code and exports all of the definitions marked `pub`.

The argument to the macro is the name of the module we'd like to create. We'll use `parser` to keep things simple but you can call it whatever you like, as long as you change all of the references in this book.

You don't need to know _how_ it does it, just what it exports.

```rust,ignore
fn main() {
    let ast = parser::ProgramParser::new()
        .parse("let foo")
        .unwrap();

    dbg!(ast);
}
```

We've told LALRPOP to create a `parser` module, so we can now interact with our `ProgramParser` that we defined in the grammar file. We create a new instance of it with the static `::new()` method and tell it to `.parse()` out a string.

It only knows how to parse `let` statements, so that's all we'll send through for now. The `.parse()` method returns a `Result<Vec<Statement>, lalrpop_util::ParseError>` so it needs to be unwrapped to retrieve the AST.

> We'll take a look at nicer error reporting later on, this will do for now.

Finally, we can dump out the generated AST with the `dbg!` macro since we added `#[Debug]` to our node structures.

If you run this code, you should see some output in the terminal:

```bash
[src/main.rs:13] ast = [
    Let {
        name: "foo",
        value: None,
    },
]
```

This means we've successfully parsed a `let` statement with LALRPOP. Give yourself a pat on the back, you did good!

With that done, we should probably write some tests! Let's write our tests in the `src/main.rs` file for now, we can move them around later on.

```rust,ignore
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    fn expect_ast(code: &str, expected: Vec<Statement>) {
        assert_eq!(
            parser::ProgramParser::new().parse(code).unwrap(),
            expected
        );
    }

    #[test]
    fn it_can_parse_a_simple_let_statement() {
        expect_ast("let foo", vec![
            Statement::Let {
                name: String::from("foo"),
                value: None,
            }
        ]);
    }
}
```

The `expect_ast()` helper function tidies this up a little bit. We're just asserting that the output from the parser matches that of our expected output.

For this to work, you'll also need to add the `PartialEq` derive to both the `ast::Statement` and `ast::Expression` enumerations, since we're doing comparisons inside of the helper function.