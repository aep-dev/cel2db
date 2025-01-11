# cel2db

A library to convert cel expressions to database queries. Available as a library
for Rust, but this project also intends to target any language that is able to call C functions.

## Why Rust exposing C?

The motivation is to provide this common functionality across multiple
languages, minimizing maintenance as much as possible.

- Rust was chosen due to it's developer experience, and type and memory safety.
- The C FFI is used to expose functionality, since almost all languages support
  some form of FFI with C.