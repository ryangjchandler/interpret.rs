# What is a parser?

If you did a computer science course at university, chances are you've spent a decent amount of time learning about parsers. If that's you, feel free to skip over this chapter.

If you _don't_ know what a parser is, keep on reading. I'll do my best to explain things simply.

## Tokens

Essentially every programming language begins by breaking your code down into tokens.

Let's assume we have a JavaScript-like language.

```js
let name = "Ryan";
```

We naturally give each word and character in the code a name. We'd call `let` a **keyword**, `name` would be an **identifier** and `"Ryan"` would be a **string**.

The names we assign to each word and character are known as of token types.

```php
T_LET       "let"
T_IDENT     "name"
T_ASSIGN    "="
T_STRING    "Ryan"
```

Keywords tend to have more specific names than `T_KEYWORD`. This becomes useful when we start processing our 

## Parsing