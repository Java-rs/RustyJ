use crate::types::*;
use std::collections::HashMap;

pub struct TypeChecker {
    classes: HashMap<String, Class>,
    pub typed_classes: HashMap<String, Class>,
    current_class: Option<Class>,
    current_typed_class: Class,
    fields: HashMap<String, Vec<FieldDecl>>,
    current_local_vars: HashMap<String, Type>,
    methods: HashMap<String, Vec<MethodDecl>>,
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
            current_typed_class: Class::default(),
            fields: HashMap::new(),
            methods: HashMap::new(),
            current_local_vars: HashMap::new(),
        })
    }

    pub fn check_program(&mut self) -> Result<(), String> {
        let classes = self.classes.clone();
        for (_, class) in &classes {
            self.current_class = Some(class.clone());

            self.check_class(class)?;
            self.fields.clear();
        }
        Ok(())
    }

    fn check_class(&mut self, class: &Class) -> Result<(), String> {
        self.current_typed_class.name = class.name.clone();

        self.fields.insert(class.name.clone(), vec![]);
        for field in &class.fields {
            self.check_field(field)?;
        }

        self.current_typed_class.fields = self.fields.get(&class.name).unwrap().clone();

        self.methods.insert(class.name.clone(), vec![]);
        for method in &class.methods {
            let types_method = self.check_method(method)?;
            self.current_typed_class.methods.push(types_method);
            self.current_local_vars.clear();
        }

        self.typed_classes
            .insert(class.name.clone(), self.current_typed_class.clone());
        Ok(())
    }

    fn check_field(&mut self, field: &FieldDecl) -> Result<(), String> {
        if self
            .check_field_type(&field.field_type, &field.val)
            .is_err()
        {
            return Err(format!(
                "Field value '{0}' does not match type '{1}'",
                field.val.clone().unwrap(),
                field.field_type
            ));
        }

        let names = self
            .fields
            .get_mut(&self.current_class.as_ref().unwrap().name.to_string())
            .unwrap();

        // Check for duplicate field names
        if names.iter().any(|vec_field| &vec_field.name == &field.name) {
            return Err(format!("Duplicate field name: {}", field.name));
        } else {
            names.push(field.clone());
        }

        Ok(())
    }

    // write a function that checks if the optional val is of the same type as the field_type
    // if it is, return Ok(())
    fn check_field_type(&self, field_type: &Type, val: &Option<String>) -> Result<(), String> {
        if let Some(val) = val {
            match field_type {
                Type::Int => {
                    if val.parse::<i32>().is_err() {
                        return Err(format!("Field type is int, but val is not int"));
                    }
                }
                Type::Bool => {
                    if val.parse::<bool>().is_err() {
                        return Err(format!("Field type is bool, but val is not bool"));
                    }
                }
                Type::Char => {
                    if val.parse::<char>().is_err() {
                        return Err(format!("Field type is char, but val is not char"));
                    }
                }
                Type::String => {
                    if val.parse::<String>().is_err() {
                        return Err(format!("Field type is string, but val is not string"));
                    }
                }
                Type::Void => {
                    return Err(format!("Field type is void, but val is not void"));
                }
                Type::Null => {
                    return Err(format!("Field type is null, but val is not null"));
                }
                // Fixme: Pls Maxi check this
                Type::Class(str) => {
                    if !self.classes.contains_key(str) {
                        return Err(format!("Field type is class, but class does not exist"));
                    }
                }
            }
        }
        Ok(())
    }

    fn check_method(&mut self, method: &MethodDecl) -> Result<MethodDecl, String> {
        method.params.iter().for_each(|(t, name)| {
            self.current_local_vars.insert(name.clone(), t.clone());
        });

        self.check_stmt(&method.body)?;

        let mut typed_method = method.clone();

        self.check_stmt(&method.body)?;
        typed_method.body = self.type_stmt(&method.body);

        let name = self.current_class.as_ref().unwrap().name.clone();

        if let Some(methods) = self.methods.get_mut(&name) {
            if methods
                .iter()
                .any(|vec_method| vec_method.name == method.name)
            {
                return Err(format!("Duplicate method name: {}", method.name));
            } else {
                methods.push(method.clone());
            }
        } else {
            self.methods.insert(name.clone(), vec![method.clone()]);
        }

        Ok(typed_method.clone())
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
        return match expr {
            Expr::This => Expr::TypedExpr(
                Box::new(Expr::This),
                Type::Class(self.current_class.as_ref().unwrap().name.clone()),
            ),
            Expr::LocalOrFieldVar(name) => {
                if let Some(t) = self.current_local_vars.get(name) {
                    return Expr::TypedExpr(
                        Box::new(Expr::LocalOrFieldVar(name.clone())),
                        t.clone(),
                    );
                }
                if let Some(field) = self
                    .current_class
                    .as_ref()
                    .unwrap()
                    .fields
                    .iter()
                    .find(|field| field.name == *name)
                {
                    return Expr::TypedExpr(
                        Box::new(Expr::LocalOrFieldVar(name.clone())),
                        field.field_type.clone(),
                    );
                }
                panic!("Unknown variable: {}", name)
            }
            Expr::InstVar(expr, name) => Expr::TypedExpr(
                Box::new(Expr::InstVar(Box::new(self.type_expr(expr)), name.clone())),
                match self.type_expr(expr) {
                    Expr::TypedExpr(_, t) => t,
                    _ => panic!("Expected typed expr"),
                },
            ),
            Expr::Unary(s, expr) => {
                let t = match self.type_expr(expr) {
                    Expr::TypedExpr(_, t) => t,
                    _ => panic!("Expected typed expr"),
                };
                let op = UnaryOp::from(s.as_str());
                match op {
                    UnaryOp::Pos => {
                        if t != Type::Int {
                            panic!("Type mismatch");
                        }
                        return Expr::TypedExpr(
                            Box::new(Expr::Unary(s.clone(), Box::new(self.type_expr(expr)))),
                            t,
                        );
                    }
                    UnaryOp::Neg => {
                        if t != Type::Int {
                            panic!("Type mismatch");
                        }
                        return Expr::TypedExpr(
                            Box::new(Expr::Unary(s.clone(), Box::new(self.type_expr(expr)))),
                            t,
                        );
                    }
                    UnaryOp::Not => {
                        if t != Type::Bool {
                            panic!("Type mismatch");
                        }
                        return Expr::TypedExpr(
                            Box::new(Expr::Unary(s.clone(), Box::new(self.type_expr(expr)))),
                            t,
                        );
                    }
                }
                Expr::TypedExpr(
                    Box::new(Expr::Unary(s.clone(), Box::new(self.type_expr(expr)))),
                    match self.type_expr(expr) {
                        Expr::TypedExpr(_, t) => t,
                        _ => panic!("Expected typed expr"),
                    },
                )
            }
            Expr::Binary(s, expr1, expr2) => {
                let op = BinaryOp::from(s.as_str());
                let t1 = match self.type_expr(expr1) {
                    Expr::TypedExpr(_, t) => t,
                    _ => panic!("Expected typed expr"),
                };
                let t2 = match self.type_expr(expr2) {
                    Expr::TypedExpr(_, t) => t,
                    _ => panic!("Expected typed expr"),
                };
                match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
                        if t1 != t2 {
                            panic!("Type mismatch");
                        }
                        return Expr::TypedExpr(
                            Box::new(Expr::Binary(
                                s.clone(),
                                Box::new(self.type_expr(expr1)),
                                Box::new(self.type_expr(expr2)),
                            )),
                            t1,
                        );
                    }
                    BinaryOp::Lt | BinaryOp::Le => {
                        if t1 != t2 {
                            panic!("Type mismatch");
                        }
                        return Expr::TypedExpr(
                            Box::new(Expr::Binary(
                                s.clone(),
                                Box::new(self.type_expr(expr1)),
                                Box::new(self.type_expr(expr2)),
                            )),
                            Type::Bool,
                        );
                    }
                    BinaryOp::Eq | BinaryOp::Ne => {
                        if t1 != t2 {
                            panic!("Type mismatch");
                        }
                        return Expr::TypedExpr(
                            Box::new(Expr::Binary(
                                s.clone(),
                                Box::new(self.type_expr(expr1)),
                                Box::new(self.type_expr(expr2)),
                            )),
                            Type::Bool,
                        );
                    }
                    _ => {}
                }
                Expr::TypedExpr(
                    Box::new(Expr::Binary(
                        s.clone(),
                        Box::new(self.type_expr(expr1)),
                        Box::new(self.type_expr(expr2)),
                    )),
                    Type::Bool,
                )
            }
            Expr::Integer(i) => Expr::TypedExpr(Box::new(Expr::Integer(*i)), Type::Int),
            Expr::Bool(b) => Expr::TypedExpr(Box::new(Expr::Bool(*b)), Type::Bool),
            Expr::Char(c) => Expr::TypedExpr(Box::new(Expr::Char(*c)), Type::Char),
            Expr::String(s) => Expr::TypedExpr(Box::new(Expr::String(s.clone())), Type::String),
            Expr::Jnull => Expr::TypedExpr(Box::new(Expr::Jnull), Type::Null),
            Expr::StmtExprExpr(stmt_expr) => Expr::TypedExpr(
                Box::new(Expr::StmtExprExpr(Box::new(self.type_stmt_expr(stmt_expr)))),
                Type::Int,
            ),
            Expr::TypedExpr(expr, t) => Expr::TypedExpr(Box::new(self.type_expr(expr)), t.clone()),
            Expr::LocalVar(expr, name) => Expr::TypedExpr(
                Box::new(Expr::LocalVar(expr.clone(), name.clone())),
                self.current_local_vars.get(name).unwrap().clone(),
            ),
        };
    }

    fn type_stmt_expr(&self, stmt_expr: &StmtExpr) -> StmtExpr {
        match stmt_expr {
            StmtExpr::Assign(name, expr) => StmtExpr::TypedStmtExpr(
                Box::new(StmtExpr::Assign(name.clone(), self.type_expr(expr))),
                Type::Int,
            ),
            StmtExpr::TypedStmtExpr(stmt_expr, t) => {
                StmtExpr::TypedStmtExpr(Box::new(self.type_stmt_expr(stmt_expr)), t.clone())
            }
            StmtExpr::New(t, exprs) => StmtExpr::TypedStmtExpr(
                Box::new(StmtExpr::New(
                    t.clone(),
                    exprs.iter().map(|e| self.type_expr(e)).collect(),
                )),
                Type::Int,
            ),
            StmtExpr::MethodCall(expr, name, exprs) => StmtExpr::TypedStmtExpr(
                Box::new(StmtExpr::MethodCall(
                    self.type_expr(expr),
                    name.clone(),
                    exprs.iter().map(|e| self.type_expr(e)).collect(),
                )),
                Type::Int,
            ),
        }
    }
    fn infer_expr_type(&self, expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::This => {
                // Here we would look up the type of 'this' in the context
                unimplemented!()
            }
            Expr::LocalOrFieldVar(name) => {
                // Here we would look up the variable in the context
                self.current_local_vars
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
