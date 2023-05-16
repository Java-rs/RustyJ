#[derive(Debug, Clone)]
struct TypedExpr {
    expr: Expr,
    typ: Type,
}

struct TypeChecker {
    class_map: HashMap<String, Class>,
}

impl TypeChecker {
    fn new(classes: Vec<Class>) -> Self {
        let class_map = classes
            .into_iter()
            .map(|class| match class.name {
                Type::Class(name) => (name, class),
                _ => panic!("Invalid class name"),
            })
            .collect();

        TypeChecker { class_map }
    }

    fn check_expr(&self, expr: Expr, context: &HashMap<String, Type>) -> TypedExpr {
        match expr {
            Expr::Integer(_) => TypedExpr {
                expr,
                typ: Type::Integer,
            },
            Expr::Bool(_) => TypedExpr {
                expr,
                typ: Type::Boolean,
            },
            Expr::Char(_) => TypedExpr {
                expr,
                typ: Type::Char,
            },
            Expr::String(_) => TypedExpr {
                expr,
                typ: Type::String,
            },
            Expr::Null => TypedExpr {
                expr,
                typ: Type::Null,
            },
            Expr::LocalOrFieldVar(name) => {
                if let Some(typ) = context.get(&name) {
                    TypedExpr {
                        expr,
                        typ: typ.clone(),
                    }
                } else {
                    panic!("Undefined variable {}", name);
                }
            }

            _ => unimplemented!(),
        }
    }

}
