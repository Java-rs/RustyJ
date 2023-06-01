use crate::typechecker::*;
use crate::types::*;
/// The DIR(Duck Intermediate Representation) is our IR for generating Java Bytecode
/// from our TAST
pub struct DIR {
    pub(crate) constant_pool: Vec<Constant>,
    pub(crate) classes: Vec<IRClass>,
}
struct ConstantPool(Vec<Constant>);
impl ConstantPool {
    fn new() -> Self {
        Self(vec![])
    }
    /// Adds a constant to the constant pool, returning its index
    fn add(&mut self, constant: Constant) -> u16 {
        if let Some(index) = self.0.iter().position(|c| *c == constant) {
            return index as u16;
        }
        self.0.push(constant);
        let index = self.0.len() as u16;
        index
    }
    /// Returns the constant at the given index. Note that this is 1-indexed since the constant
    /// pool of the JVM is 1-indexed
    fn get(&self, index: u16) -> Option<&Constant> {
        self.0.get(index as usize - 1)
    }
}
pub(crate) struct IRClass {
    pub(crate) name: String,
    pub(crate) super_name: String,
    pub(crate) fields: Vec<FieldDecl>,
    pub(crate) methods: Vec<CompiledMethod>,
}

impl IRClass {
    pub(crate) fn new(
        name: String,
        fields: Vec<FieldDecl>,
        methods: Vec<CompiledMethod>,
    ) -> IRClass {
        IRClass {
            name,
            super_name: String::from("java/lang/Object"),
            fields,
            methods,
        }
    }
}

#[derive(Debug)]
pub struct IRFieldDecl {
    pub(crate) type_index: u16,
    pub(crate) access_flags: AccessFlags,
    pub(crate) name_index: u16,
}
#[repr(u8)]
#[derive(Debug)]
pub(crate) enum AccessFlags {
    Public,
}

impl IRFieldDecl {
    pub(crate) fn new(type_index: u16, name_index: u16) -> IRFieldDecl {
        IRFieldDecl {
            type_index,
            access_flags: AccessFlags::Public,
            name_index,
        }
    }
}

pub(crate) struct CompiledMethod {
    pub(crate) name: String,
    pub(crate) max_stack: u16,
    pub(crate) code: Vec<Instruction>,
}
#[derive(PartialEq, Eq, Debug)]
pub(crate) enum Constant {
    Class(String),
    FieldRef(String),
    MethodRef(String),
    NameAndType(String),
    Utf8(String),
}
pub(crate) enum Instruction {
    aload(u8),
    iload(u8),
    ifeq(u16),
    Return,
}

pub fn generate_dir(ast: &Prg) -> DIR {
    let mut dir = DIR {
        constant_pool: vec![],
        classes: vec![],
    };
    for class in ast {
        dir.classes.push(generate_class(class, &dir));
    }
    dir
}

fn generate_class(class: &Class, dir: &DIR) -> IRClass {
    let mut ir_class = IRClass::new(class.name.clone(), vec![], vec![]);
    for field in &class.fields {
        ir_class.fields.push(field.clone());
    }
    for method in &class.methods {
        ir_class.methods.push(generate_method(method, dir));
    }
    ir_class
}

fn generate_field(field: &FieldDecl, constant_pool: &mut ConstantPool) -> IRFieldDecl {
    let name_index = constant_pool.add(Constant::Utf8(field.name.clone()));
    // FIXME: The type is wrong. This will yield int, char etc. instead of I, C etc.
    let type_index = constant_pool.add(Constant::Utf8(field.field_type.clone().to_string()));
    IRFieldDecl::new(type_index, name_index)
}

// TODO: Parallelize this, since methods are not dependent on each other(hopefully)
fn generate_method(method: &MethodDecl, dir: &DIR) -> CompiledMethod {
    let mut compiled_method = CompiledMethod {
        name: method.name.clone(),
        max_stack: 0,
        code: vec![],
    };

    compiled_method
        .code
        .append(&mut generate_code_stmt(method.body.clone(), dir));

    compiled_method
}

fn generate_code_stmt(stmt: Stmt, dir: &DIR) -> Vec<Instruction> {
    match stmt {
        Stmt::Block(stmts) => {
            // TODO: Bene
        }
        Stmt::Return(expr) => {
            // Generate bytecode for return
            // TODO: Mary
        }
        Stmt::While(expr, stmt) => {
            // Generate bytecode for while
            // TODO: Bene
        }
        Stmt::LocalVarDecl(types, name) => {
            // Generate bytecode for local var decl
            // TODO: Mary
        }
        Stmt::If(expr, stmt1, stmt2) => {
            // Generate bytecode for if
            // TODO: Bene
        }
        Stmt::StmtExprStmt(stmt_expr) => {
            // Generate bytecode for stmt expr
            // TODO: Mary
        }
        Stmt::TypedStmt(stmt, types) => {
            // Generate bytecode for typed stmt
            // TODO: Bene
        }
    }
    vec![]
}

fn generate_code_stmt_expr(stmt_expr: &StmtExpr, code: &mut Vec<u8>) {
    match stmt_expr {
        StmtExpr::Assign(name, expr) => {
            // Generate bytecode for assignment
        }
        StmtExpr::New(types, exprs) => {
            // Generate bytecode for new
        }
        StmtExpr::MethodCall(expr, name, exprs) => {
            // Generate bytecode for method call
        }
        StmtExpr::TypedStmtExpr(stmt_expr, types) => {
            // Generate bytecode for typed stmt expr
        }
    }
}

fn generate_code_expr(expr: &Expr, code: &mut Vec<u8>) {
    match expr {
        Expr::This => {
            // Generate bytecode for this
        }
        Expr::LocalOrFieldVar(name) => {
            // Generate bytecode for local or field var
        }
        Expr::InstVar(expr, name) => {
            // Generate bytecode for inst var
        }
        Expr::Unary(op, expr) => {
            // Generate bytecode for unary
        }
        Expr::Binary(op, expr1, expr2) => {
            // Generate bytecode for binary
        }

        Expr::Integer(value) => {
            // Generate bytecode for int literal
        }
        Expr::Bool(value) => {
            // Generate bytecode for bool literal
        }
        Expr::Char(value) => {
            // Generate bytecode for char literal
        }
        Expr::String(value) => {
            // Generate bytecode for string literal
        }
        Expr::Jnull => {
            // Generate bytecode for null literal
        }
        Expr::StmtExprExpr(stmt_expr) => {
            // Generate bytecode for stmt expr
        }
        Expr::TypedExpr(expr, types) => {
            // Generate bytecode for typed expr
        }
        Expr::LocalVar(expr, name) => {
            // Generate bytecode for local var
        }
    }
}
fn generate_code_type(types: &Type, code: &mut Vec<u8>) {
    match types {
        Type::Int => {
            // Generate bytecode for int
        }
        Type::Bool => {
            // Generate bytecode for bool
        }
        Type::Char => {
            // Generate bytecode for char
        }
        Type::String => {
            // Generate bytecode for string
        }
        Type::Void => {
            // Generate bytecode for void
        }
        Type::Null => {
            // Generate bytecode for null
        }
        Type::Class(name) => {
            // Generate bytecode for class
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_fields() {
        let mut constant_pool = ConstantPool::new();
        let field = FieldDecl {
            field_type: Type::Int,
            name: String::from("test"),
        };
        let ir_field = generate_field(&field, &mut constant_pool);
        assert_eq!(ir_field.name_index, 1);
        assert_eq!(ir_field.type_index, 2);
        assert_eq!(
            constant_pool.get(1),
            Some(&Constant::Utf8(String::from("test")))
        );
        assert_eq!(
            constant_pool.get(2),
            Some(&Constant::Utf8(String::from("int")))
        );
    }
}
