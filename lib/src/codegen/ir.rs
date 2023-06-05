use crate::typechecker::*;
use crate::types;
use crate::types::*;
use std::io::Bytes;

/// The DIR(Duck Intermediate Representation) is our IR for generating Java Bytecode
/// from our TAST
pub struct DIR {
    pub(crate) constant_pool: ConstantPool,
    pub(crate) classes: Vec<IRClass>,
}
impl DIR {
    /// Because this involves crating the constant pool, this is a mutable method
    pub fn as_bytes(&mut self) -> Vec<u8> {
        let mut result = vec![0xCA, 0xFE, 0xBA, 0xBE];
        // Minor version, always 0
        result.extend_from_slice(&[0, 0]);
        // Major version, always 52
        result.extend_from_slice(&[0, 52]);
        // Constant pool count. For some unknow reason, this is 1-indexed and we have to add 1 to
        // the size
        result.extend_from_slice(&(self.constant_pool.0.len() as u16 + 1).to_be_bytes());
        // Constant pool
        result.append(&mut self.constant_pool.as_bytes());
        result.extend_from_slice(&self.constant_pool.as_bytes());
        // TODO: Functions
        result
    }
}
pub struct ConstantPool(Vec<Constant>);
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
    /// See this table: https://docs.oracle.com/javase/specs/jvms/se15/html/jvms-4.html#jvms-4.4:w
    fn as_bytes(&mut self) -> Vec<u8> {
        let mut result = vec![];
        // TODO: Remove this clone and act on self reference instead
        for constant in self.0.clone() {
            match constant {
                Constant::Utf8(val) => {
                    result.push(1);
                    // Len is 2 bytes large
                    result.extend_from_slice(&(val.len() as u16).to_be_bytes());
                    result.extend_from_slice(val.as_bytes());
                }
                Constant::Class(name) => {
                    result.push(7);
                    result.extend_from_slice(
                        &(self.add(Constant::Utf8(name.clone())) as u16).to_be_bytes(),
                    );
                }
                Constant::MethodRef(MethodRef { class, method }) => {
                    result.push(10);
                    result.extend_from_slice(
                        &(self.add(Constant::Class(class)) as u16).to_be_bytes(),
                    );
                    result.extend_from_slice(
                        &(self.add(Constant::NameAndType(method)) as u16).to_be_bytes(),
                    );
                }
                Constant::NameAndType(NameAndType { name, r#type }) => {
                    result.push(12);
                    result
                        .extend_from_slice(&(self.add(Constant::Utf8(name)) as u16).to_be_bytes());
                    result.extend_from_slice(
                        &(self.add(Constant::Utf8(r#type)) as u16).to_be_bytes(),
                    );
                }
                Constant::FieldRef(FieldRef { class, field }) => {
                    result.push(9);
                    result.extend_from_slice(
                        &(self.add(Constant::Class(class)) as u16).to_be_bytes(),
                    );
                    result.extend_from_slice(
                        &(self.add(Constant::NameAndType(field)) as u16).to_be_bytes(),
                    );
                }
            }
        }
        result
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
#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) enum Constant {
    Class(String),
    FieldRef(FieldRef),
    /// This has to be of format class_name.method_name. If it is later found to be beneficial however we could split this into two Strings
    MethodRef(MethodRef),
    NameAndType(NameAndType),
    Utf8(String),
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FieldRef {
    class: String,
    field: NameAndType,
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MethodRef {
    class: String,
    method: NameAndType,
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct NameAndType {
    name: String,
    r#type: String,
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
        constant_pool: ConstantPool(vec![]),
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
    let type_index = constant_pool.add(Constant::Utf8(
        field.field_type.clone().to_ir_string().into(),
    ));
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
            &mut stmts
                .iter()
                .map(|stmt| generate_code_stmt(stmt.clone(), dir, local_var_pool))
                // Flatten to avoid vecs in our vec
                .flatten()
                .collect(),
        ),
        Stmt::Return(expr) => match expr {
            Integer => result.push(Instruction::ireturn),
            Boolean => result.push(Instruction::ireturn),
            Char => result.push(Instruction::ireturn),
            String => result.push(Instruction::areturn),
            types::Expr::Jnull => result.push(Instruction::areturn),
            _ => panic!("Invalid return type"),
        },
        Stmt::While(expr, stmt) => {
            // TODO: Test, Bene
            result.append(&mut generate_code_expr(expr));
            // Generate bytecode for our body
            let mut body = generate_code_stmt(*stmt, dir, local_var_pool);
            result.push(Instruction::reljumpifeq(body.len() as i16));
            result.append(&mut body);
            result.push(Instruction::reljumpifeq(-(body.len() as i16)));
        }
        Stmt::LocalVarDecl(types, name) => {
            let index: u8 = local_var_pool.0.len() as u8 + 1;
            //TODO: fix bipush to "store" the value of the variable
            match types {
                types::Type::Int => result.append(&mut vec![
                    Instruction::bipush(index),
                    Instruction::istore(index),
                    Instruction::iload(index),
                ]),
                Boolean => result.append(&mut vec![
                    Instruction::bipush(index),
                    Instruction::istore(index),
                    Instruction::iload(index),
                ]),
                types::Type::Char => result.append(&mut vec![
                    Instruction::bipush(index),
                    Instruction::istore(index),
                    Instruction::iload(index),
                ]),
                types::Type::String => result.append(&mut vec![
                    Instruction::bipush(index),
                    Instruction::astore(index),
                    Instruction::aload(index),
                ]),
                types::Type::Null => result.append(&mut vec![
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
            result.append(&mut generate_code_expr(expr));
            // We set a label to jump to if the expression is false
            let mut if_stmt = generate_code_stmt(*stmt1, dir, local_var_pool);
            // If the expression is false, jump to the else block
            result.push(Instruction::reljumpifeq(if_stmt.len() as i16));
            // If the expression is true, execute the if block
            result.append(&mut if_stmt);
            // If there is an else block, execute it
            if let Some(stmt) = stmt2 {
                result.append(&mut generate_code_stmt(*stmt, dir, local_var_pool));
            }
        }
        Stmt::StmtExprStmt(stmt_expr) => {
            result.append(&mut generate_code_stmt_expr(&stmt_expr));
        }
        Stmt::TypedStmt(stmt, _types) => {
            // Generate bytecode for typed stmt
            // TODO: Check whether we can actually generate the same code as a normal stmt
            result.append(&mut generate_code_stmt(*stmt, &dir, local_var_pool));
        }
    }
    return result;
}

fn generate_code_stmt_expr(stmt_expr: &StmtExpr) -> Vec<Instruction> {
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

fn generate_code_expr(expr: Expr) -> Vec<Instruction> {
    let mut result = vec![];
    todo!();
    result
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
