use crate::typechecker::*;
use crate::types::*;
use std::io::Bytes;
/// The DIR(Duck Intermediate Representation) is our IR for generating Java Bytecode
/// from our TAST
pub struct DIR {
    pub(crate) constant_pool: Vec<Constant>,
    pub(crate) classes: Vec<IRClass>,
}
impl DIR {
    pub const fn as_bytes(&self) -> &[u8] {
        vec![].as_slice()
    }
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
struct LocalVarPool(Vec<(String, u8)>);

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
    bipush(u8),       //Push byte onto stack
    istore(u8),       //Store int into local variable
    astore(u8),       //Store reference into local variable
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
    let mut local_var_pool = LocalVarPool(vec![]);
    let mut compiled_method = CompiledMethod {
        name: method.name.clone(),
        max_stack: 0,
        code: vec![],
    };

    compiled_method.code.append(&mut generate_code_stmt(
        method.body.clone(),
        dir,
        &mut local_var_pool,
    ));

    compiled_method
}

fn generate_code_stmt(
    stmt: Stmt,
    dir: &DIR,
    local_var_pool: &mut LocalVarPool,
) -> Vec<Instruction> {
    let mut result = vec![];
    match stmt {
        Stmt::Block(stmts) => result.append(
            stmts
                .iter()
                .map(|stmt| generate_code_stmt(stmt.clone(), dir))
                .collect(),
        ),
        Stmt::Return(expr) => match expr {
            Integer => result.push(Instruction::ireturn),
            Boolean => result.push(Instruction::ireturn),
            Char => result.push(Instruction::ireturn),
            String => result.push(Instruction::areturn),
            Jnull => result.push(Instruction::areturn),
            _ => panic!("Invalid return type"),
        },
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
            index = local_var_pool.0.len() + 1;
            //TODO: fix bipush to "store" the value of the variable
            match types {
                Int => result.append(&mut vec![
                    Instruction::bipush(index),
                    Instruction::istore(index),
                    Instruction::iload(index),
                ]),
                Boolean => result.append(&mut vec![
                    Instruction::bipush(index),
                    Instruction::istore(index),
                    Instruction::iload(index),
                ]),
                Char => result.append(&mut vec![
                    Instruction::bipush(index),
                    Instruction::istore(index),
                    Instruction::iload(index),
                ]),
                String => result.append(&mut vec![
                    Instruction::bipush(index),
                    Instruction::astore(index),
                    Instruction::aload(index),
                ]),
                Null => result.append(&mut vec![
                    Instruction::bipush(index),
                    Instruction::astore(index),
                    Instruction::aload(index),
                ]),
                _ => panic!("Invalid return type"),
            }
            local_var_pool.0.push((name, index));
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
            result.append(generate_code_stmt_expr(stmt_expr));
        }
        Stmt::TypedStmt(stmt, _types) => {
            // Generate bytecode for typed stmt
            // TODO: Check whether we can actually generate the same code as a normal stmt
            result.append(generate_code_stmt(stmt));
        }
    }
    return result;
}

fn generate_code_stmt_expr(stmt_expr: &StmtExpr) -> Vec<u8> {
    match stmt_expr {
        StmtExpr::Assign(name, expr) => {
            // Generate bytecode for assignment
            // TODO: Bene
        }
        StmtExpr::New(types, exprs) => {
            // Generate bytecode for new
            // TODO: Mary
        }
        StmtExpr::MethodCall(expr, name, exprs) => {
            // Generate bytecode for method call
            // TODO: Bene
        }
        StmtExpr::TypedStmtExpr(stmt_expr, types) => {
            // TODO: Mary
            // Generate bytecode for typed stmt expr
        }
    }
    vec![]
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
