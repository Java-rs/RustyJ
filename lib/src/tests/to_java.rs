use crate::types::*;

pub fn get_indents(amount: u8) -> String {
    let mut s = "".to_string();
    let mut i: u8 = 0;
    while i < amount {
        s += "\t";
        i += 1;
    }
    s
}

pub fn prg_to_java(prg: &Prg) -> String {
    let mut s: String = String::new();
    for class in prg {
        s = format!("{}\n", class_to_java(class));
    }
    s
}

pub fn class_to_java(class: &Class) -> String {
    let mut s: String = format!("class {} ", class.name);
    s += "{\n";
    for field in &class.fields {
        s = format!("{}{}", s, field_to_java(&field));
    }
    for method in &class.methods {
        s = format!("{}{}", s, method_to_java(&method));
    }
    s += "}\n";
    s
}

pub fn field_to_java(field: &FieldDecl) -> String {
    let mut s: String = format!("\t{} {}", field.field_type, field.name);
    if let Some(x) = &field.val {
        s = format!("{} = {}", s, expr_to_java(x));
    }
    s += ";\n";
    s
}

pub fn method_to_java(method: &MethodDecl) -> String {
    let mut s: String = format!(
        "\t{} {}({}) ",
        method.ret_type,
        method.name,
        method
            .params
            .clone()
            .into_iter()
            .map(|p| format!("{} {}", p.0, p.1))
            .reduce(|acc, s| format!("{}, {}", acc, s))
            .unwrap_or("".to_string())
    );
    s += "{\n";
    s = format!("{}{}", s, stmt_to_java(&method.body, 2));
    s += "\t}\n";
    s
}

pub fn stmt_to_java(stmt: &Stmt, indent: u8) -> String {
    match stmt {
        Stmt::Block(stmts) => stmts
            .into_iter()
            .map(|stmt| stmt_to_java(stmt, indent))
            .fold("".to_string(), |acc, s| acc + &s),
        Stmt::If(cond, body, elze) => {
            let mut s = format!(
                "{}if ({}) {{\n{}{}}}",
                get_indents(indent),
                expr_to_java(cond),
                stmt_to_java(body, indent + 1),
                get_indents(indent)
            );
            if let Some(x) = elze {
                s += &format!(
                    " else {{\n{}{}}}",
                    stmt_to_java(x, indent + 1),
                    get_indents(indent)
                )
            }
            s += "\n";
            s
        }
        Stmt::LocalVarDecl(typ, name) => format!("{}{} {};\n", get_indents(indent), typ, name),
        Stmt::Return(expr) => format!("{}return {};\n", get_indents(indent), expr_to_java(expr)),
        Stmt::StmtExprStmt(stmt_expr) => {
            format!("{}{};\n", get_indents(indent), stmt_expr_to_java(stmt_expr))
        }
        Stmt::TypedStmt(stmt, typ) => stmt_to_java(stmt, indent),
        Stmt::While(cond, body) => format!(
            "{}while ({}) {{\n{}{}}}\n",
            get_indents(indent),
            expr_to_java(cond),
            stmt_to_java(body, indent + 1),
            get_indents(indent)
        ),
    }
}

pub fn params_to_java(params: &Vec<Expr>) -> String {
    params
        .into_iter()
        .map(|expr| expr_to_java(expr))
        .reduce(|acc, s| format!("{}, {}", acc, s))
        .unwrap_or(String::new())
}

pub fn stmt_expr_to_java(stmt_expr: &StmtExpr) -> String {
    match stmt_expr {
        StmtExpr::Assign(var, expr) => format!("{} = {}", var, expr_to_java(expr)),
        StmtExpr::MethodCall(expr, name, params) => format!(
            "{}.{}({})",
            expr_to_java(expr),
            name,
            params_to_java(params)
        ),
        StmtExpr::New(typ, params) => format!("new {}({})", typ, params_to_java(params)),
        StmtExpr::TypedStmtExpr(stmt_expr, typ) => stmt_expr_to_java(stmt_expr),
    }
}

pub fn expr_to_java(expr: &Expr) -> String {
    match expr {
        Expr::Binary(op, l, r) => format!("({}) {} ({})", expr_to_java(l), op, expr_to_java(r)),
        Expr::Bool(b) => b.to_string(),
        Expr::Char(c) => {
            println!("{}", format!("{c}"));
            format!("'{}'", c)
        }
        Expr::InstVar(expr, var) => format!("{}.{}", expr_to_java(expr), var),
        Expr::Integer(i) => i.to_string(),
        Expr::Jnull => "null".to_string(),
        Expr::LocalOrFieldVar(var) => var.to_owned(),
        Expr::LocalVar(var) => var.to_owned(),
        Expr::FieldVar(var) => var.to_owned(),
        Expr::StmtExprExpr(stmt_expr) => stmt_expr_to_java(stmt_expr),
        Expr::String(s) => {
            println!("{}", format!("{s}"));
            format!("\"{}\"", s)
        }
        Expr::This => "this".to_string(),
        Expr::TypedExpr(expr, typ) => expr_to_java(expr),
        Expr::Unary(op, expr) => format!("{}({})", op, expr_to_java(expr)),
    }
}
