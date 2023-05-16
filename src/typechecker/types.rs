#[derive(Debug, Clone)]
enum Type {
    Integer,
    Boolean,
    Char,
    String,
    Null,
    Class(String),
}

#[derive(Debug, Clone)]
enum Expr {
    This,
    Super,
    LocalOrFieldVar(String),
    InstVar(Box<Expr>, String),
    Unary(String, Box<Expr>),
    Binary(String, Box<Expr>, Box<Expr>),
    Integer(i32),
    Bool(bool),
    Char(char),
    String(String),
    Null,
    StmtExprExpr(Box<StmtExpr>),
}

#[derive(Debug, Clone)]
enum StmtExpr {
    Assign(String, Expr),
    New(Type, Vec<Expr>),
    MethodCall(Expr, String, Vec<Expr>),
}

#[derive(Debug, Clone)]
enum Stmt {
    Block(Vec<Stmt>),
    Return(Expr),
    While(Expr, Box<Stmt>),
    LocalVarDecl(Type, String),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    StmtExprStmt(StmtExpr),
}

#[derive(Debug, Clone)]
struct MethodDecl {
    return_type: Type,
    name: String,
    params: Vec<(Type, String)>,
    body: Stmt,
}

#[derive(Debug, Clone)]
struct FieldDecl {
    field_type: Type,
    name: String,
}

#[derive(Debug, Clone)]
struct Class {
    name: Type,
    fields: Vec<FieldDecl>,
    methods: Vec<MethodDecl>,
}

type Prg = Vec<Class>;
