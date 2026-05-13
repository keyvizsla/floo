# Docs

The documentation for floo is built using [mdbook](https://rust-lang.github.io/mdBook/index.html).

## Prerequisites

To be able to build the documentation as html, make sure to have the following installed:
- `mdbook` 
- `mdbook-variables`

## Building the docs

Building the documentation is as easy as running `mdbook build`.
To run a local server with hot reloading (nice when you are actively working on the documentation)
simply run `mdbook serve --open`, which will automatically open a connection to the live server
in your default web browser.
There are many more things you can do with `mdbook`, make sure to consult the [documentation](https://rust-lang.github.io/mdBook/index.html).
