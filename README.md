# Concrete
Concrete is programming language designed to write safe maintainable massively scalable systems. It's design will be heavily influenced by Erlang, SML/OCaml, Rust and Austral. We also want to incorporate some ideas from Elm, Pony, Lua, Clojure and Go.

[Telegram](https://t.me/concrete_proglang)

## Design

### Features
- Very simple C/Go-inspired, context-free grammar, syntax: if, for, function calls, modules, pattern matching
- Linear type system
    - Type inference only within blocks, not in function signatures.
    - Algebraic Data Types
- Capabilities
- Pluggable concurrency runtime with a preemptive scheduler, green threads and copy only message passing, a REPL and excellent observability
- Profiling and tracing as first class citizens
- Explicit over implicit
- Easily embeddable
- Compile to MLIR, WASM and generate C code
- Implemented in Rust

### Anti-features
- No garbage collection
- No default runtime
- No macros
- No exceptions

## Inspiration
- Standard ML
- Rust
- Erlang & Elixir
- Go
- Roc https://www.roc-lang.org/
- Austral https://austral-lang.org/spec/spec.html
- Odin https://odin-lang.org/
- Vale https://vale.dev/
