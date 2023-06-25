use super::*;

pub fn stmt_tast_to_ast(stmt: &Stmt) -> Stmt {
    match stmt {
        Stmt::TypedStmt(x, typ) => stmt_tast_to_ast(x),
        Stmt::Block(stmts) => Block(stmts.iter().map(|x| stmt_tast_to_ast(x)).collect()),
        Stmt::Return(expr) => Return(expr_tast_to_ast(expr)),
        Stmt::While(cond, body) => While(expr_tast_to_ast(cond), Box::new(stmt_tast_to_ast(body))),
        Stmt::If(cond, body, elze) => If(
            expr_tast_to_ast(cond),
            Box::new(stmt_tast_to_ast(body)),
            match elze {
                Some(x) => Some(Box::new(stmt_tast_to_ast(x))),
                None => None,
            },
        ),
        Stmt::StmtExprStmt(stmt_expr) => StmtExprStmt(stmt_expr_tast_to_ast(stmt_expr)),
        default => stmt.clone(),
    }
}

pub fn stmt_expr_tast_to_ast(stmt_expr: &StmtExpr) -> StmtExpr {
    match stmt_expr {
        StmtExpr::Assign(var, val) => Assign(var.clone(), expr_tast_to_ast(val)),
        StmtExpr::New(typ, params) => New(
            typ.clone(),
            params.iter().map(|x| expr_tast_to_ast(x)).collect(),
        ),
        StmtExpr::MethodCall(obj, method, params) => MethodCall(
            expr_tast_to_ast(obj),
            method.clone(),
            params.iter().map(|x| expr_tast_to_ast(x)).collect(),
        ),
        StmtExpr::TypedStmtExpr(x, typ) => stmt_expr_tast_to_ast(x),
    }
}

pub fn expr_tast_to_ast(expr: &Expr) -> Expr {
    match expr {
        Expr::InstVar(x, s) => InstVar(Box::new(expr_tast_to_ast(x)), s.clone()),
        Expr::Unary(s, x) => Unary(s.clone(), Box::new(expr_tast_to_ast(x))),
        Expr::Binary(op, l, r) => Binary(
            op.clone(),
            Box::new(expr_tast_to_ast(l)),
            Box::new(expr_tast_to_ast(r)),
        ),
        Expr::StmtExprExpr(x) => StmtExprExpr(Box::new(stmt_expr_tast_to_ast(x))),
        Expr::TypedExpr(x, t) => expr_tast_to_ast(x),
        default => expr.clone(),
    }
}

pub fn tast_to_ast(class: &Class) -> Class {
    Class {
        name: class.name.clone(),
        fields: class.fields.clone(),
        methods: class
            .methods
            .clone()
            .into_iter()
            .map(|method| MethodDecl {
                ret_type: method.ret_type.clone(),
                name: method.name.clone(),
                params: method.params.clone(),
                body: stmt_tast_to_ast(&method.body),
            })
            .collect(),
    }
}