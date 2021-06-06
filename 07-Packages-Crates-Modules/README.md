# This is a package for a restaraunt

Cargo has four broad concepts in the module system:

Packages
: Highest level, builds, tests, and shares one or more crates

Crates
: A tree of modules that produces a library AND/OR executable.

Modules (and use)
: Control the organization, scope, and privacy of paths

Paths
: A way of naming an item such as a struct, function, or module (think namespaces)

## Packages and Crates

A package is a collection of crates, which contains a `Cargo.toml` file that
describes how to to build the crates. It must contain at least one crate, but
it can only contain at most 1 (one) library crate. We can have as many binary
crates as we want.

Binary Crate
: executable crate that the user interacts with directly and will be signified
  by a `src/main.rs`, but multiple can live in `src/bin/`.

Library Crate
: modules to be shared with other rust programs that is signified by the 
  presence of `src/lib.rs`.

> At the moment, this project only contains a Binary Crate.

## Modules and Control Scope

`use` keyword
: Brings a path into scope

`pub` keyword
: make items public


