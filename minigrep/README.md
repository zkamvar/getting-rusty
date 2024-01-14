## A mini version of grep

Note: a rust version of grep called `ripgrep` already exists and is fantastic.

The purpose of this exercise is to get me writing an application using techniques
that I learned from the Rust Book:

 - Organizing code (using what you learned about modules in Chapter 7)
 - Using vectors and strings (collections, Chapter 8)
 - Handling errors (Chapter 9)
 - Using traits and lifetimes where appropriate (Chapter 10)
 - Writing tests (Chapter 11)

This will be a cli program that will be run from `cargo run --` with
two arguments:

```
minigrep: steps to get a grep

USAGE:
    cargo run -- <string> <file>

ARGUMENTS:
    string  a search string (not regex because I ain't that fancy)
    file    path to a file that you want to search in

EXAMPLE:
    
    cargo run -- 'grep' README.md
```
