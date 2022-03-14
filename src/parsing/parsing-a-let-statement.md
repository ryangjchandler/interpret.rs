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