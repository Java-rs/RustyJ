## Supported types
- Welche Types, Strukturen und deren Kombinationen werden unterstützt?

## Parser

-   Wer hat welche Arbeit am Parser gemacht?
-   Wie wurde Pest fürs Parsing eingesetzt? (sehr ähnlich zu ANTLR)

## Typchecker

Geschrieben von: Maximilian Floto, Philipp Wolf

Der Typechecker akzeptiert einen Abstract Syntax Tree (AST) und gibt einen getypten AST (TAST) zurück.
Er führt eine umfassende Analyse durch, um die Typen aller Variablen und Ausdrücke im Code zu bestimmen.
Er stellt sicher, dass jede Variable vor ihrer Verwendung korrekt deklariert und initialisiert wurde, wodurch potenzielle Fehler in der Anwendung vermieden werden.
Er bestimmt die Typen aller Ausdrücke und Variablen und überprüft, ob alle Variablen vor ihrer Verwendung korrekt deklariert und initialisiert wurden.

Funktionsweise des Typecheckers:
Der Typechecker iteriert über alle übergebenen Klassen und prüft auf mehrfache Klassendeklarationen. Die Felddeklarationen werden in einem neuen getypten Klassenobjekt gespeichert, in dem alle weiteren getypten Methoden und deren Statements gespeichert werden. Anschließend iteriert der Typechecker über alle Methoden und prüft auf mehrfache Methodendeklarationen und typisiert die Methodenparameter. Nachdem alle Statements typisiert wurden, wird der Rückgabetyp der Methode geprüft und die Methode im getypten Klassenobjekt gespeichert. Nach dem Überprüfen und Typisieren der Klasse wird diese in einen Vektor an getypten Klassen gespeichert. Nachdem alle Klassen getypt wurden, wird der Vektor an getypten Klassen zurückgegeben.

Folgende Funktionen werden vom Typechecker übernommen:
-  Liest alle definierten Types/Strukturen
-  Typisierung aller Variablen und Ausdrücke
-  Checken von mehreren Klassen
-  Checken der Rückgabe-Typen von Methoden
-  Ersetzen von LocalOrFieldVar durch LocalVar oder FieldVar


Folgende Fehler werden vom Typechecker erkannt:
-  Mehrfache Deklaration einer Klasse
-  Mehrfache Deklaration einer Methode
-  Type-Mismatch bei Methodenrückgabe
-  Type-Mismatch bei Methodenaufruf
-  Type-Mismatch bei Methodenparametern
-  Type-Mismatch bei FieldDecl
-  Type-Mismatch bei Unary/Binary-Operationen
-  Mehrfache Deklaration von FieldDecl
-  Mehrfache Deklaration von LocalOrFieldVar
-  Nicht deklarierte Variable
-  Unbekannte Methode bei Methodenaufruf
-  Verwendung einer nicht deklarierten Variable
-  Panic bei TypedExpr in AST
-  Bedingung von If/While-Statement ist kein Bool

## Codegenerierung

Geschrieben von: Marion Hinkel, Benedikt Brandmaier, Val Richter
-   Wer hat welche Arbeit bei codegen gemacht?
-   Was für Arbeit musste alles zusätzlich getan werden, weil wir nicht Java + ASM genutzt haben?

## Testing

-   Wer hat welche Arbeit beim Testing gemacht?
-   Wie funktioniert das Testing genau?

## AST-Definition

-   Wer hat wie zur endgültigen Definition des ASTs beigetragen?
-   v.a. interessant, dass während der Arbeit der einzelnen Teams, der AST iterativ verändert wurde

## Projektmanagement

-   Welche Arbeit lief von wem ins Projektmanagement?
