# RustyJ

A MiniJava Compiler written in Rust.

## MiniJava

MiniJava is a subset of Java. It is a simple object-oriented language that supports classes, single inheritance(not
supported in this compiler), and strong typing.

## Build

### Requirements

-   [Rust](https://rustup.rs/)

### Build

```bash
cargo build --release
```

## Usage

```bash
cargo r -r -- <input_file> [<output_file>]
```

## Documentation

A more detailed documentation on how to use the separate parts of the compiler is given in [User-Doc](./docs/User-Doc.md). A detailed documentation on how the project was done is given in [Project-Doc](./docs/Project-Doc.md).

## Disclaimer

This compiler was done as a student project and doesn't support many language features and may contain bugs. It is not intended to be used in production.

However, it was a great learning experience, and we hope it can be useful to someone else.

## Authors

-   Philipp Wolf (Flippchen)
-   Maximilian Floto (mfloto)
-   Marion Hinkel (Segelente)
-   Benedikt Brandmaier (Nereuxofficial)
-   Tori Gönnheimer (ToriTheGreenOne)
-   Sander Stella (SanderForGodot)
-   Val Richter (ArtInLines)
