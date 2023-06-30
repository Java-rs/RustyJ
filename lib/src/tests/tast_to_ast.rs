use super::*;

pub fn stmt_tast_to_ast(stmt: &Stmt) -> Stmt {
    match stmt {
        Stmt::TypedStmt(x, _typ) => stmt_tast_to_ast(x),
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
        _ => stmt.clone(),
    }
}

pub fn stmt_expr_tast_to_ast(stmt_expr: &StmtExpr) -> StmtExpr {
    match stmt_expr {
        StmtExpr::Assign(var, val) => Assign(expr_tast_to_ast(var), expr_tast_to_ast(val)),
        StmtExpr::New(typ, params) => New(
            typ.clone(),
            params.iter().map(|x| expr_tast_to_ast(x)).collect(),
        ),
        StmtExpr::MethodCall(obj, method, params) => MethodCall(
            expr_tast_to_ast(obj),
            method.clone(),
            params.iter().map(|x| expr_tast_to_ast(x)).collect(),
        ),
        StmtExpr::TypedStmtExpr(x, _typ) => stmt_expr_tast_to_ast(x),
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
        Expr::TypedExpr(x, _t) => expr_tast_to_ast(x),
        Expr::LocalVar(v) => Expr::LocalOrFieldVar(v.clone()),
        Expr::FieldVar(v) => Expr::LocalOrFieldVar(v.clone()),
        _ => expr.clone(),
    }
}

pub fn tast_to_ast(class: &Class) -> Class {
    Class {
        name: class.name.clone(),
        fields: class
            .fields
            .iter()
            .map(|field| FieldDecl {
                field_type: field.field_type.clone(),
                name: field.name.clone(),
                val: field.val.clone().and_then(|x| Some(expr_tast_to_ast(&x))),
            })
            .collect(),
        methods: class
            .methods
            .iter()
            .map(|method| MethodDecl {
                ret_type: method.ret_type.clone(),
                name: method.name.clone(),
                params: method.params.clone(),
                body: stmt_tast_to_ast(&method.body),
            })
            .collect(),
    }
}
