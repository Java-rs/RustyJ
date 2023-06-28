TBD

- Jeweils erwähnen wie man die Teile kompiliert und laufen lassen kann. Nicht vergessen: Wie sollte die Eingabe aussehen und wie sieht die Ausgabe dann aus?
- Vielleicht sollten wir Command-Line Befehle für jeden der Teile im Binary Crate zur Verfügung stellen?

## Vorbedingungen
[Rust installieren](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
# Ausführen
Um eine .java in eine .class-datei zu kompilieren:
```bash
cargo run -- <input_file>
```

# Testen
1. Projekt bauen: `cargo build`

## Parser

2. Parser tests ausführen: `cargo test test_parser`
## Typchecker

2. Typechecker tests ausführen: `cargo test test_typechecker`

Spezifischen Test ausführen: `cargo test <test_name>::test_typechecker`

## Codegenerierung

2. Codegenerierung tests ausführen: `cargo test test_codegen`

## Testing
Zur ausführung aller tests: `cargo test`