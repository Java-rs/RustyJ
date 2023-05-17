use crate::types::*;
use std::collections::HashMap;

pub struct TypeChecker {
    classes: HashMap<String, Class>,
    current_class: Option<Class>,
    field_names: HashMap<String, Vec<String>>,
}

impl TypeChecker {
    pub fn new(program: Prg) -> Self {
        let mut classes = HashMap::new();
        for class in program {
            classes.insert(class.0.clone().to_string(), class.clone());
        }
        Self {
            classes,
            current_class: None,
            field_names: HashMap::new(),
        }
    }

    pub fn check_program(&mut self) -> Result<(), String> {
        let classes = self.classes.clone();
        for (_, class) in &classes {
            self.current_class = Some(class.clone());
            self.check_class(class)?;
            self.field_names.clear();
        }
        Ok(())
    }

    fn check_class(&mut self, class: &Class) -> Result<(), String> {
        for field in &class.1 {
            self.check_field(field)?;
        }
        for method in &class.2 {
            self.check_method(method)?;
        }
        Ok(())
    }

    fn check_field(&mut self, field: &FieldDecl) -> Result<(), String> {
        if let Some(names) = self
            .field_names
            .get_mut(&self.current_class.as_ref().unwrap().0.to_string())
        {
            if names.contains(&field.1) {
                return Err(format!("Duplicate field name: {}", field.1));
            } else {
                names.push(field.1.clone());
            }
        } else {
            self.field_names.insert(
                self.current_class.as_ref().unwrap().0.clone().to_string(),
                vec![field.1.clone()],
            );
        }
        Ok(())
    }

    fn check_method(&self, method: &MethodDecl) -> Result<(), String> {
        self.check_stmt(&method.3)
    }

    fn check_stmt(&self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Return(expr) => self.check_expr(expr),
            Stmt::While(expr, stmt) => {
                self.check_expr(expr)?;
                self.check_stmt(stmt)
            }
            Stmt::If(expr, stmt1, stmt2) => {
                self.check_expr(expr)?;
                self.check_stmt(stmt1)?;
                if let Some(s) = stmt2 {
                    self.check_stmt(s)?;
                }
                Ok(())
            }
            Stmt::Block(stmts) => {
                for stmt in stmts {
                    self.check_stmt(stmt)?;
                }
                Ok(())
            }
            Stmt::LocalVarDecl(_, _) => Ok(()),
            Stmt::StmtExprStmt(stmt_expr) => self.check_stmt_expr(stmt_expr),
            _ => Ok(()),
        }
    }

    fn check_expr(&self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::LocalOrFieldVar(name) => {
                let class = self.current_class.as_ref().ok_or("No current class")?;
                if !class.1.iter().any(|field| field.1 == *name) {
                    return Err(format!("Unknown variable: {}", name));
                }
                Ok(())
            }
            Expr::Binary(_, expr1, expr2) => {
                self.check_expr(expr1)?;
                self.check_expr(expr2)?;
                Ok(())
            }
            Expr::StmtExprExpr(stmt_expr) => self.check_stmt_expr(stmt_expr),
            Expr::InstVar(expr, _) => self.check_expr(expr),
            Expr::Unary(_, expr) => self.check_expr(expr),
            Expr::Integer(_) => Ok(()),
            Expr::Bool(_) => Ok(()),
            Expr::Char(_) => Ok(()),
            Expr::String(_) => Ok(()),
            Expr::Jnull => Ok(()),
            Expr::This => Ok(()),
            Expr::Super => Ok(()),
            _ => Ok(()),
        }
    }

    fn check_stmt_expr(&self, stmt_expr: &StmtExpr) -> Result<(), String> {
        match stmt_expr {
            StmtExpr::Assign(name, expr) => {
                let class = self.current_class.as_ref().ok_or("No current class")?;
                if !class.1.iter().any(|field| field.1 == *name) {
                    return Err(format!("Unknown variable: {}", name));
                }
                self.check_expr(expr)
            }
            StmtExpr::New(_, exprs) => {
                for expr in exprs {
                    self.check_expr(expr)?;
                }
                Ok(())
            }
            StmtExpr::MethodCall(expr, _, exprs) => {
                self.check_expr(expr)?;
                for expr in exprs {
                    self.check_expr(expr)?;
                }
                Ok(())
            }
            StmtExpr::TypedStmtExpr(expr, _) => {
                !unimplemented!();
                Ok(())
            }
        }
    }
}
