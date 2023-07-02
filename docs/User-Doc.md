## Vorbedingungen

[Rust installieren](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

-   [JDK 20](https://www.oracle.com/java/technologies/downloads/) (Wird in Tests für die Validierung des Codegens benötigt)

# Ausführen

Um eine .java in eine .class-datei zu kompilieren:

```bash
cargo r -r -- <input_file> [<output_file>]
```

# Testen

1. Projekt bauen: `cargo build`

## Parser

2. Parser Tests ausführen: `cargo test --lib test_parser`

Spezifischen Test ausführen: `cargo test --lib <test_name>::test_parser`

## Typchecker

2. Typechecker Tests ausführen: `cargo test --lib test_typechecker`

Spezifischen Test ausführen: `cargo test --lib <test_name>::test_typechecker`

## Codegenerierung

2. Codegenerierung Tests ausführen: `cargo test --lib test_codegen`

Spezifischen Test ausführen: `cargo test --lib <test_name>::test_codegen`

## TAST

2. Ausführung der Tests von den handgeschriebenen TASTs: `cargo test --lib test_class`

Spezifischen Test ausführen: `cargo test --lib <test_name>::test_class`

## Testing

Zur ausführung aller Tests: `cargo test --lib`. Dabei sollte beachtet werden, dass die Tests teilweise die selben Dateien schreiben und entsprechend Probleme aufkommen können, wenn alle Tests gleichzeitig ausgeführt werden. Diese Probleme treten nicht auf, wenn die Teile des Compilers (Parser, Typchecker, Codegenerierung) einzeln getestet werden.
