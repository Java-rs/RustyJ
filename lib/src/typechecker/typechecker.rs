use crate::types::*;
use std::collections::HashMap;

pub struct TypeChecker {
    classes: HashMap<String, Class>,
    typed_classes: HashMap<String, Class>,
    current_class: Option<Class>,
    current_typed_class: Option<Class>,
    field_names: HashMap<String, Vec<String>>,
    current_method_vars: HashMap<String, Type>,
    method_names: Vec<String>,
}

impl TypeChecker {
    pub fn new(program: Prg) -> Result<Self, String> {
        let mut class_names = Vec::new();
        let mut classes = HashMap::new();
        for class in program {
            // Check for duplicate class names
            if class_names.contains(&class.name) {
                return Err(format!("Duplicate class name: {}", class.name));
            } else {
                class_names.push(class.name.clone());
            }

            classes.insert(class.name.clone().to_string(), class.clone());
        }
        Ok(Self {
            classes,
            typed_classes: HashMap::new(),
            current_class: None,
            current_typed_class: None,
            field_names: HashMap::new(),
            method_names: Vec::new(),
            current_method_vars: HashMap::new(),
        })
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
        for field in &class.fields {
            self.check_field(field)?;
        }
        for method in &class.methods {
            self.check_method(method)?;
        }
        Ok(())
    }

    fn check_field(&mut self, field: &FieldDecl) -> Result<(), String> {
        if let Some(names) = self
            .field_names
            .get_mut(&self.current_class.as_ref().unwrap().name.to_string())
        {
            if names.contains(&field.name) {
                return Err(format!("Duplicate field name: {}", field.name));
            } else {
                names.push(field.name.clone());
            }
        } else {
            self.field_names.insert(
                self.current_class
                    .as_ref()
                    .unwrap()
                    .name
                    .clone()
                    .to_string(),
                vec![field.name.clone()],
            );
        }
        Ok(())
    }

    fn check_method(&mut self, method: &MethodDecl) -> Result<(), String> {
        // Check for duplicate method names
        let name = &method.name;
        if self.method_names.contains(name) {
            return Err(format!("Duplicate method name: {}", name));
        } else {
            self.method_names.push(name.clone());
        }

        method.params.iter().for_each(|(t, name)| {
            self.current_method_vars.insert(name.clone(), t.clone());
        });
        self.check_stmt(&method.body);
        self.current_method_vars.clear();
        self.check_stmt(&method.body)
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
                if !class.fields.iter().any(|field| field.name == *name)
                    && !class
                        .methods
                        .iter()
                        .any(|method| method.params.iter().any(|(_, var_name)| var_name == name))
                {
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
                if !class.fields.iter().any(|field| field.name == *name) {
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
            _ => Ok(()),
        }
    }

    fn type_stmt(&self, stmt: &Stmt) -> Stmt {
        match stmt {
            Stmt::Block(stmts) => {
                let typed_stmts = stmts.iter().map(|s| self.type_stmt(s)).collect();
                Stmt::TypedStmt(Box::new(Stmt::Block(typed_stmts)), Type::Int)
            }
            Stmt::Return(expr) => {
                Stmt::TypedStmt(Box::new(Stmt::Return(self.type_expr(expr))), Type::Int)
            }
            Stmt::While(expr, stmt) => Stmt::TypedStmt(
                Box::new(Stmt::While(
                    self.type_expr(expr),
                    Box::new(self.type_stmt(stmt)),
                )),
                Type::Int,
            ),
            Stmt::LocalVarDecl(t, name) => Stmt::TypedStmt(
                Box::new(Stmt::LocalVarDecl(t.clone(), name.clone())),
                Type::Int,
            ),
            Stmt::If(expr, stmt1, stmt2) => Stmt::TypedStmt(
                Box::new(Stmt::If(
                    self.type_expr(expr),
                    Box::new(self.type_stmt(stmt1)),
                    stmt2.as_ref().map(|s| Box::new(self.type_stmt(s))),
                )),
                Type::Int,
            ),
            Stmt::StmtExprStmt(stmt_expr) => Stmt::TypedStmt(
                Box::new(Stmt::StmtExprStmt(self.type_stmt_expr(stmt_expr))),
                Type::Int,
            ),
            Stmt::TypedStmt(stmt, t) => Stmt::TypedStmt(Box::new(self.type_stmt(stmt)), t.clone()),
        }
    }

    fn type_expr(&self, expr: &Expr) -> Expr {
        match expr {
            Expr::This => Expr::TypedExpr(Box::new(Expr::This), Type::Int),
            Expr::Super => Expr::TypedExpr(Box::new(Expr::Super), Type::Int),
            Expr::LocalOrFieldVar(name) => {
                Expr::TypedExpr(Box::new(Expr::LocalOrFieldVar(name.clone())), Type::Int)
            }
            Expr::InstVar(expr, name) => Expr::TypedExpr(
                Box::new(Expr::InstVar(Box::new(self.type_expr(expr)), name.clone())),
                Type::Int,
            ),
            Expr::Unary(s, expr) => Expr::TypedExpr(
                Box::new(Expr::Unary(s.clone(), Box::new(self.type_expr(expr)))),
                Type::Int,
            ),
            Expr::Binary(s, expr1, expr2) => Expr::TypedExpr(
                Box::new(Expr::Binary(
                    s.clone(),
                    Box::new(self.type_expr(expr1)),
                    Box::new(self.type_expr(expr2)),
                )),
                Type::Int,
            ),
            Expr::Integer(i) => Expr::TypedExpr(Box::new(Expr::Integer(*i)), Type::Int),
            Expr::Bool(b) => Expr::TypedExpr(Box::new(Expr::Bool(*b)), Type::Int),
            Expr::Char(c) => Expr::TypedExpr(Box::new(Expr::Char(*c)), Type::Int),
            Expr::String(s) => Expr::TypedExpr(Box::new(Expr::String(s.clone())), Type::Int),
            Expr::Jnull => Expr::TypedExpr(Box::new(Expr::Jnull), Type::Int),
            Expr::StmtExprExpr(stmt_expr) => Expr::TypedExpr(
                Box::new(Expr::StmtExprExpr(Box::new(self.type_stmt_expr(stmt_expr)))),
                Type::Int,
            ),
            Expr::TypedExpr(expr, t) => Expr::TypedExpr(Box::new(self.type_expr(expr)), t.clone()),
        }
    }

    fn type_stmt_expr(&self, stmt_expr: &StmtExpr) -> StmtExpr {
        match stmt_expr {
            StmtExpr::Assign(name, expr) => StmtExpr::TypedStmtExpr(
                Box::new(StmtExpr::Assign(
                    name.clone(),
                    Box::new(self.type_expr(expr)),
                )),
                Type::Int,
            ),
            StmtExpr::TypedStmtExpr(stmt_expr, t) => {
                StmtExpr::TypedStmtExpr(Box::new(self.type_stmt_expr(stmt_expr)), t.clone())
            }
        }
    }
    fn infer_expr_type(&self, expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::This => {
                // Here we would look up the type of 'this' in the context
                unimplemented!()
            }
            Expr::Super => {
                // Here we would look up the type of 'super' in the context
                unimplemented!()
            }
            Expr::LocalOrFieldVar(name) => {
                // Here we would look up the variable in the context
                self.current_method_vars
                    .get(name)
                    .cloned()
                    .ok_or(format!("Variable '{}' not found", name))
            }
            Expr::InstVar(expr, name) => {
                // Here we would look up the type of the instance variable
                unimplemented!()
            }
            Expr::Unary(s, expr) => {
                // Depending on your language's semantics, unary operations might always result in a specific type
                // Or they might depend on the type of the operand
                unimplemented!()
            }
            Expr::Binary(s, expr1, expr2) => {
                // Binary operations might result in a specific type or depend on the operands
                // For example, an addition might result in the same type as the operands if they are of the same type,
                // or it might result in an error if they are not
                let type1 = self.infer_expr_type(expr1)?;
                let type2 = self.infer_expr_type(expr2)?;

                if type1 != type2 {
                    return Err(format!(
                        "Mismatched types in binary operation: {} and {}",
                        type1, type2
                    ));
                }

                Ok(type1)
            }
            Expr::Integer(_) => Ok(Type::Int),
            Expr::Bool(_) => Ok(Type::Bool),
            Expr::Char(_) => Ok(Type::Char),
            Expr::String(_) => Ok(Type::String),
            Expr::Jnull => {
                // Here you might want to return a specific 'null' type, or perhaps an optional type parameter
                unimplemented!()
            }
            Expr::StmtExprExpr(stmt_expr) => self.infer_stmt_expr_type(stmt_expr),
            _ => unimplemented!(),
        }
    }

    fn infer_stmt_expr_type(&self, stmt_expr: &StmtExpr) -> Result<Type, String> {
        match stmt_expr {
            StmtExpr::Assign(name, expr) => {
                // In many languages, an assignment results in the same type as the right hand side
                self.infer_expr_type(expr)
            }
            _ => unimplemented!(),
        }
    }
}
