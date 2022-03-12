# The Language

Both of our interpreter architectures will implement the same language. For simplicity, we'll be implementing a language that looks a lot like JavaScript.

You can name your language anything you like. This book will refer to the language as SimpleScript.

Below is a small block of code that demonstrates some of SimpleScript's features:

```js
// This is a variable declaration.
let name = "Ryan";

// This is a function definition.
function fib(n) {
    // Conditionals are simple.
    if n < 2 {
        return n;
    } else {
        // Returns are explicit.
        return fib(n - 1) + fib(n - 2);
    }
}

// Function invocation looks like any C-style language.
fib(30);
```

Here's a simpler list of things that are language will be able to do:

* Declare global variables, as well as local variables.
* Define functions that can be called / invoked.
* Conditional statements (`if`)
* Loops (`while`)
* Generic mathematical operations (`+`, `-`, `*`, `/`, `%`)

Our language will also have the following data types:

* `number` - represents both integers and 64-bit floating-point decimals.
* `string`
* `bool`
* `array` - a list of values of any type.
* `null`