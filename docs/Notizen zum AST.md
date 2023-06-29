# Notizen zum AST

Die Definition des ASTs wurde stark von den Ansätzen aus der Vorlesung inspiriert. Im Folgenden sollen v.a. anders 
getroffene Entscheidungen begründet werden. Zusätzlich soll kurz erklärt werden, wie das Diagramm genau zu lesen ist.
Das ist nötig, weil die Programmiersprache, die zur Erstellung des Compilers genutzt wurde, nicht objekt-orientiert ist 
und eine Übersetzung von UML-Diagramm zur Definition im Code somit leicht anders als bei Java Programmen ist.

## Erklärung des Diagramms

Einige Klassen des Diagramms haben einen blauen Rand. 
Das zeigt an, dass diese Elemente noch nicht beim Parsen erstellt werden, sondern erst beim Typcheck hinzugefügt werden. 
Speziell werden alle Statements, Expression und Statement-Expressions in TypedStmt, TypedExpr bzw. TypedStmtExpr gepackt. 
Somit kann Typ-Information hinzugefügt werden, die beim Parsing noch nicht vorhanden war. Auch werden alle `LocalOrFieldVar`
Objekte bei der Typisierung in LocalVar bzw. FieldVar Objekte transformiert. Der Grund dahinter liegt darin, dass der 
Typchecker sowieso herausfinden muss, ob es sich bei der Variable um eine lokale Variable oder ein Attribut der Klasse 
handelt. Da die Codegenerierung diese Informationen ebenfalls benötigt, werden die dann beim Typchecker direkt in den 
AST geliefert.

Im Quellcode werden viele der "Klassen" des Diagramms als Enumerations dargestellt. Das liegt daran, dass die gewählte 
Programmiersprache - [Rust](https://www.rust-lang.org/) - sehr mächtige Enumerations zur Verfügung stellt. 
Bei Rust kann jeder Variante aus einem Enum eine eigene Menge an Parametern gegeben werden. Somit können alle Attribute 
der Klasse als Parameter des Enums abgebildet werden. Speziell wurde das bei der Definition von `Stmt`, `Expr` und 
`StmtExpr` genutzt. Da alle diese Elemente nicht wirklich von eine Überklasse erben, sondern eigentlich Varianten eines 
Enums sind, wurden sie im Diagramm als einzelne Klassen in einer Box, die das Enum darstellt, abgebildet.

## Erklärung unserer Entscheidungen

- Assign nimmt eine expression für die Variable auf, da wir bei der Codegenerierung wissen müssen um welchen Typ an Variable es sich handelt, da dort dann unterschiedlicher Bytecode generiert werden muss
- FieldDecl nimmt einen optionalen Wert zur direkten Zuweisung auf
- Type wurde um Class erweitert um die Codegenerierung zu vereinfachen
