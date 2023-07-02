## Supported types
- Welche Types, Strukturen und deren Kombinationen werden unterstützt?

## Parser

Geschrieben von: Victoria Gönnheimer, Sander Stella

Der Parser akzeptiert denn text eines Java programs und gibt einen Abstract Syntax Tree (AST) zurück.
Dafür wird die Libray [pest.rs](https://pest.rs/) verwendet um das Inital parsing durchzufüren.
Für dieses inital parsing nutzt pest unsere vorher definiete Gramatik. Bei der Gramatik wurde sich primär and der vorlesugn orientiert mit signifikaten abänderungen um das parsing zuvereinfachen sowie den spezifikationen der library nachzukommen.
Das egebnis welches Pest zurückgibt wird von uneren parser funktionen analysiert und umgewandelt.
Dabei wird wie folgt vorgegangenen:
- Eine funktion schaut sich die aktuelle regel an
- Es wird der entsprechende code zu dieser regelausgefür
- Sofern subregeln in dieser regel vorkommen wird die entsprechende funktion aufgerufen


## Typechecker

Geschrieben von: Maximilian Floto und Philipp Wolf im Pair Programming

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

ByteCode-Umwandlung, Bugfixes, StackSize und viele Improvements: Val Richter

Definition DIR(Duck Intermediate Representation), ConstantPool, LocalVarPool, Methoden zur Instruction-generierung, BugFixes, etwas ByteCode-Umwandlung und Umwandlung relativer in absolute Jumps: Marion Hinkel und Benedikt Brandmaier im Pair Programming

Zur Bytecode-generierung wird der Typed Abstract Syntax Tree(TAST) in Java Bytecode komplett selber
umgewandelt. Dafür wird eine Intermediate Representation (IR) genutzt, die eine Class-ähnliche Struktur(mit Konstantenpool, LocalVarpool, Methoden mit Code als Instruktionen, etc.)
besitzt. Diese IR wird dann komplett manuell in Java Bytecode übersetzt. Dies hat dem Code-gen Team sehr viel
Zeit gekostet, da z.B. die Stack-Size, der Konstantenpool, LocalVarpool und die Jumps manuell berechnet werden mussten.

Zudem hatten wir zeitweise eigene Relative Instructions implementiert, da wir dachten, dass die JVM keine relativen Jumps
unterstützt, hatten dann allerdings mit Try-and-Error herausgefunden, dass javap sich die absoluten Addressen ausrechnet
und für die JVM normale jumps als relative Jumps behandelt. Auch die relativen Jumps waren aber sehr fehleranfällig und
hatten häufig off-by-one Errors.

Zudem musste eine StackMapTable implementiert werden, da die JVM sonst unsere Klassen nicht lädt.
Das Troubleshooten von Testfehlern war auch sehr aufwending da oft javap gar nicht erst den Fehler im Klassencode ausgab
und wir mit einem Hex-Editor die Klassen von Hand analysieren mussten, da es auch kein anderes Tool gab, um solche Fehler 
auszugeben und die Zeit fehlte ein Eigenes zu schreiben.

Da es auch keine Dokumentation gibt, die in etwa zeigt, welcher Bytecode für welche Operationen genutzt wird, mussten wir
uns die Bytecode-Spezifikationen anschauen und sehr viel mit Tools wie javap und [godbolt](https://godbolt.org/) arbeiten
in die wir manuell Java Code eingeben und schauten, was für Bytecode bei verschiedenen Operationenkombinationen generiert
wird, was sehr zeitaufwendig war.

Auch sehr schwierig war die Implementation einer StackMapTable, da Java diese erwartet. Diese ist eine Tabelle, die für
jede Instruktion die Typen der Elemente auf dem Stack in komprimiertem Format angibt. Diese Tabelle muss manuell 
erstellt werden und über die Typen aller Variablen, die in den Stack geschrieben wurden Bescheid wissen.

## Testing

-   Wer hat welche Arbeit beim Testing gemacht?
-   Wie funktioniert das Testing genau?

Das Testen des Codegens war sehr aufwendig, er besteht aus diesen Schritten:
1. Ein handgeschriebener TAST wird geladen
2. Eine Java Klasse wird erstellt die jede Methode im TAST aufruft
3. Die java Klasse wird mit javac kompiliert und ausgeführt, wobei die Ausgabe in einer Variable gespeichert wird
4. Fürs Debugging wird die Java Klasse mit javap in Bytecode umgewandelt und in eine Datei geschrieben
5. Der TAST wird in eine DIR umgewandelt und zu Bytecode umgewandelt
6. Der Bytecode wird in eine .class-Datei geschrieben
7. Die .class-Datei wird mit javap in Bytecode umgewandelt und in eine Datei geschrieben
8. Die vom Codegen generierte .class-Datei wird ausgeführt und die Ausgabe in einer Variable gespeichert
9. Die Ausgaben der richtigen Java Klasse und der vom Codegen generierten Klasse werden verglichen

## AST-Definition

-   Wer hat wie zur endgültigen Definition des ASTs beigetragen?
-   v.a. interessant, dass während der Arbeit der einzelnen Teams, der AST iterativ verändert wurde

## Projektmanagement

-   Welche Arbeit lief von wem ins Projektmanagement?
