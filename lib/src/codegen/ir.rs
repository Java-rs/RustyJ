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

/// The instructions for the JVM
/// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-6.html#jvms-6.5.areturn
pub(crate) enum Instruction {
    aload(u8),        //Load reference from local variable
    iload(u8),        //Load int from local variable
    ifeq(u16),        //Branch if int is 0
    ireturn,          //return int, char, boolean
    r#return,         //return void
    areturn,          //return object(string, integer, null)
    iload_1,          //Load int from local variable 1
    iload_2,          //Load int from local variable 2
    iload_3,          //Load int from local variable 3
    aload_1,          //Load reference from local variable 1
    aload_2,          //Load reference from local variable 2
    aload_3,          //Load reference from local variable 3
    bipush(u8),       //Push byte onto stack
    istore_1,         //Store int into local variable 1
    istore_2,         //Store int into local variable 2
    istore_3,         //Store int into local variable 3
    astore_1,         //Store reference into local variable 1
    astore_2,         //Store reference into local variable 2
    astore_3,         //Store reference into local variable 3
    reljumpifeq(i16), //relative jump, useful for if, while etc. Has i16 because it can jump backwards and it gets converted to u8 later
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
/// If this method is used the caller has to still set a NameAndType constant and a FieldRef
/// constant, which is technically optional if the field is not used but we're lazy
fn generate_field(field: &FieldDecl, constant_pool: &mut ConstantPool) -> IRFieldDecl {
    let name_index = constant_pool.add(Constant::Utf8(field.name.clone()));
    let type_index = constant_pool.add(Constant::Utf8(field.field_type.clone().to_ir_string()));
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

struct LocalVarPool(Vec<(String, u8)>);

fn generate_code_stmt(stmt: Stmt, dir: &DIR) -> Vec<Instruction> {
    let mut result = vec![];
    match stmt {
        Stmt::Block(stmts) => result.append(
            stmts
                .iter()
                .map(|stmt| generate_code_stmt(stmt.clone(), dir))
                .collect(),
        ),
        Stmt::Return(expr) => {
            match expr {
                //TODO: Fix numbers so its not zero
                Integer => result.push(Instruction::ireturn),
                Boolean => result.push(Instruction::ireturn),
                Char => result.push(Instruction::ireturn),
                String => result.push(Instruction::areturn),
                Jnull => result.push(Instruction::areturn),
                _ => panic!("Invalid return type"),
            }
        }
        Stmt::While(expr, stmt) => {
            // TODO: Test, Bene
            result.append(generate_code_expr(expr));
            // Generate bytecode for our body
            let body = generate_code_stmt(stmt, dir);
            result.push(Instruction::reljumpifeq(body.len() as i16));
            result.append(&mut body);
            result.push(Instruction::reljumpifeq(-(body.len() as i16)));
        }
        Stmt::LocalVarDecl(types, name) => {
            match types {
                Int => result.append(&mut vec![
                    Instruction::bipush(0),
                    Instruction::istore_1,
                    Instruction::iload_1,
                ]),
                Boolean => result.append(&mut vec![
                    Instruction::bipush(0),
                    Instruction::istore_1,
                    Instruction::iload_1,
                ]),
                Char => result.append(&mut vec![
                    Instruction::bipush(0),
                    Instruction::istore_1,
                    Instruction::iload_1,
                ]),
                String => result.append(&mut vec![
                    Instruction::bipush(0),
                    Instruction::astore_1,
                    Instruction::aload_1,
                ]),
                Null => result.append(&mut vec![
                    Instruction::bipush(0),
                    Instruction::astore_1,
                    Instruction::aload_1,
                ]),
                _ => panic!("Invalid return type"),
            }
            // Generate bytecode for local var decl
            // TODO: Mary
        }
        Stmt::If(expr, stmt1, stmt2) => {
            // Generate bytecode for if
            // TODO: Bene, testing
            // Evaluate the expression
            result.append(generate_code_expr(expr));
            // We set a label to jump to if the expression is false
            let if_stmt = generate_code_stmt(stmt1, dir);
            // If the expression is false, jump to the else block
            result.push(Instruction::reljumpifeq(if_stmt.len() as i16));
            // If the expression is true, execute the if block
            result.append(if_stmt);
            // If there is an else block, execute it
            if let Some(stmt) = stmt2 {
                result.append(generate_code_stmt(stmt, dir));
            }
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
    return result;
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

fn generate_code_expr(expr: &Expr) -> Vec<u8> {
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
    vec![]
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
