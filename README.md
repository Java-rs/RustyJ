# RustyJ

A MiniJava Compiler written in Rust.

## MiniJava

MiniJava is a subset of Java. It is a simple object-oriented language that supports classes, single inheritance(not 
supported in this compiler), and strong typing.

## Build

### Requirements

- [Rust](https://rustup.rs/)

### Build

```bash
cargo build --release
```

## Usage

```bash
cargo r -r -- <input_file> <output_file>
```

# Disclaimer
This compiler was done as a student project and doesn't support many language features and may contain bugs. It is not intended to be used in production.

However, it was a great learning experience, and we hope it can be useful to someone else.

## Parsing
Parsing is done using a [Pest grammar](https://pest.rs/).

## Type Checking
I do not know what to write about type checking as i am clueless

## Code Generation
Code generation is done manually to generate Java Bytecode. The code generation uses two passes. 
The first pass generates all instructions and the second one is used to convert relative jumps into absolute jumps.

## Authors
- Flippchen
- Marion
- mfloto
- Nereuxofficial
- Sander
- Tori
- Val
