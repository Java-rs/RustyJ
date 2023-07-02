## Vorbedingungen

[Rust installieren](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

-   [JDK 20](https://www.oracle.com/java/technologies/downloads/) (Wird in Tests für die Validierung des Codegens benötigt)
-   `java`, `javac` und `javap` müssen dabei für die Tests der Codegenerierung im Pfad sein, damit diese vom Test genutzt werden können.

# Ausführen

Um eine .java in eine .class-datei zu kompilieren:

```bash
cargo r -r -- <input_file> [<output_file>]
```

# Testen

1. Projekt bauen: `cargo build`

## Parser

2. Parser tests ausführen: `cargo test --lib test_parser`

Spezifischen Test ausführen: `cargo test --lib <test_name>::test_parser`

## Typchecker

2. Typechecker tests ausführen: `cargo test --lib test_typechecker`

Spezifischen Test ausführen: `cargo test --lib <test_name>::test_typechecker`

## Codegenerierung

2. Codegenerierung tests ausführen: `cargo test --lib test_codegen`

Spezifischen Test ausführen: `cargo test --lib <test_name>::test_codegen`

## Testing

Zur ausführung aller tests: `cargo test --lib`. Dabei sollte beachtet werden, dass die Tests teilweise die selben Dateien schreiben und entsprechend Probleme aufkommen können, wenn alle Tests auf einmal ausgeführt werden. Diese Probleme treten nicht auf, wenn die Teile des Compilers (Parser, Typchecker, Codegenerierung) einzeln getestet werden.
