#![allow(non_camel_case_types)]

use crate::typechecker::*;
use crate::types;
use crate::types::Expr::Binary;
use crate::types::*;
use std::any::TypeId;
use std::fmt::Debug;
use std::io::Bytes;

/// The DIR(Duck Intermediate Representation) is our IR for generating Java Bytecode
/// from our TAST
#[derive(Debug)]
pub struct DIR {
    pub(crate) constant_pool: ConstantPool,
    pub(crate) classes: Vec<IRClass>,
}
impl DIR {
    /// Because this involves crating the constant pool, this is a mutable method
    /// https://docs.oracle.com/javase/specs/jvms/se15/html/jvms-4.html#jvms-4.1
    /// Since we have a DIR we can assume the methods have been expanded into Vectors of Instructions
    pub fn as_bytes(&mut self) -> Vec<u8> {
        // TODO: We should really check if all of these have the right size. Most lengths are u16
        // Only one class for now
        let current_class = &self.classes[0];
        let mut result = vec![0xCA, 0xFE, 0xBA, 0xBE];
        // Minor version, always 0
        result.extend_from_slice(&[0, 0]);
        // Major version, always 52
        result.extend_from_slice(&[0, 52]);
        // Add this_class and super class to constant pool. Super class is always java/lang/Object
        let this_class_index = self
            .constant_pool
            .add(Constant::Class(current_class.name.clone()));
        let super_class_index = self
            .constant_pool
            .add(Constant::Class("java/lang/Object".to_string()));
        let mut field_infos = current_class
            .fields
            .iter()
            .map(|f| f.as_bytes(&mut self.constant_pool))
            .flatten()
            .collect();
        // Constant pool count. For some unknown reason, this is 1-indexed and we have to add 1 to
        // the size
        result.extend_from_slice(&(self.constant_pool.0.len() as u16 + 1).to_be_bytes());
        // Constant pool. All constants should be present here, otherwise they will NOT be in the
        // resulting bytecode
        result.append(&mut self.constant_pool.as_bytes());
        result.extend_from_slice(&self.constant_pool.as_bytes());
        // The class access flags are just gonna be public
        result.extend_from_slice(&[0, 1]);
        result.extend_from_slice(&(this_class_index as u16).to_be_bytes());
        result.extend_from_slice(&(super_class_index as u16).to_be_bytes());
        // Interfaces count, being 0
        result.append(&mut [0, 0].to_vec());
        // Field count and fields
        result.extend_from_slice(&(current_class.fields.len() as u16).to_be_bytes());
        result.append(&mut field_infos);
        result.extend_from_slice(&(current_class.methods.len() as u16).to_be_bytes());
        // TODO: Method info
        for method in &current_class.methods {
            result.append(&mut method.as_bytes());
        }
        // Attributes count. TODO: Put Methods in here
        result.extend_from_slice(&[0, 0]);
        result
    }
}
#[derive(Debug)]
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
                    //TODO: Maybe this should be moved
                    result.push(9);
                    result.extend_from_slice(
                        &(self.add(Constant::Class(class)) as u16).to_be_bytes(),
                    );
                    result.extend_from_slice(
                        &(self.add(Constant::NameAndType(field)) as u16).to_be_bytes(),
                    );
                }
                Constant::String(val) => {
                    result.push(8);
                    result
                        .extend_from_slice(&(self.add(Constant::String(val)) as u16).to_be_bytes());
                }
            }
        }
        result
    }
}
#[derive(Debug)]
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
    fn as_bytes(&self) -> Vec<u8> {
        let mut result = vec![];
        // TODO
        result
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
struct LocalVarPool(Vec<String>);
impl LocalVarPool {
    pub fn add(&mut self, name: String) -> u16 {
        println!("Adding local var {:?}", name);
        self.0.push(name);
        self.0.len() as u16
    }
    pub fn get_index(&self, name: &str) -> u16 {
        //TODO: This is not working or saving does not work
        self.0
            .iter()
            .position(|n| n == name)
            .map(|i| i as u16)
            .expect(&*format!("Local var {:?} not found in  {:?}", name, self.0))
    }
}
#[derive(Debug)]
pub(crate) struct CompiledMethod {
    pub(crate) name: String,
    pub(crate) return_type: Type,
    pub(crate) max_stack: u16,
    pub(crate) code: Vec<Instruction>,
}

impl CompiledMethod {
    /// Get the method info as raw bytes as described in https://docs.oracle.com/javase/specs/jvms/se15/html/jvms-4.html#jvms-4.6
    fn get_method_info(&self, constant_pool: &mut ConstantPool) -> Vec<u8> {
        let mut result = vec![];
        // Access flags, always public
        result.extend_from_slice(&[0, 1]);
        // Name index
        result.extend_from_slice(
            &(constant_pool.add(Constant::Utf8(self.name.clone())) as u16).to_be_bytes(),
        );
        // Descriptor index. ()V for void, ()I for int and ()Z for bool because java developers are insane
        result.extend_from_slice(
            &(constant_pool.add(Constant::Utf8(format!(
                "(){}",
                self.return_type.to_ir_string()
            ))) as u16)
                .to_be_bytes(),
        );
        // TODO: Attributes count and attributes
        result
    }
    fn as_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) enum Constant {
    Class(String),
    FieldRef(FieldRef),
    /// This has to be of format class_name.method_name. If it is later found to be beneficial however we could split this into two Strings
    MethodRef(MethodRef),
    NameAndType(NameAndType),
    String(u16),
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
#[derive(Debug)]
pub(crate) enum Instruction {
    aload_0,
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
    aconst_null,      //Push null onto stack
    ldc(u16),         //Push item from constant pool onto stack
    ineg,             //Negate int
    goto(u16),        //Jump to instruction
    relgoto(i16),     //Jump to instruction relative to current instruction
    ifne(u16),        //Branch if int is not 0
    reljumpifne(i16), //relative jump, useful for if, while etc. Has i16 because it can jump backwards and it gets converted to u8 later
    reljumpiflt(i16), //relative jump, useful for if, while etc. Has i16 because it can jump backwards and it gets converted to u8 later
    reljumpifge(i16), //relative jump, useful for if, while etc. Has i16 because it can jump backwards and it gets converted to u8 later
    iadd,             //Add int
    isub,             //Subtract int
    imul,             //Multiply int
    idiv,             //Divide int
    irem,             //Remainder int
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
    let mut constant_pool = ConstantPool::new();
    let mut ir_class = IRClass::new(class.name.clone(), vec![], vec![]);
    for field in &class.fields {
        ir_class.fields.push(field.clone());
    }
    for method in &class.methods {
        ir_class
            .methods
            .push(generate_method(method, dir, &mut constant_pool));
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

/// Generates a Vector of instructions for a given method
fn generate_method(
    method: &MethodDecl,
    dir: &DIR,
    constant_pool: &mut ConstantPool,
) -> CompiledMethod {
    let mut local_var_pool = LocalVarPool(vec![]);
    let mut compiled_method = CompiledMethod {
        return_type: method.ret_type.clone(),
        name: method.name.clone(),
        max_stack: 0,
        code: vec![],
    };

    compiled_method.code.append(&mut generate_code_stmt(
        method.body.clone(),
        dir,
        constant_pool,
        &mut local_var_pool,
    ));

    compiled_method
}

fn generate_code_stmt(
    stmt: Stmt,
    dir: &DIR,
    constant_pool: &mut ConstantPool,
    local_var_pool: &mut LocalVarPool,
) -> Vec<Instruction> {
    let mut result = vec![];
    match stmt {
        Stmt::Block(stmts) => result.append(
            &mut stmts
                .iter()
                .map(|stmt| generate_code_stmt(stmt.clone(), dir, constant_pool, local_var_pool))
                // Flatten to avoid vecs in our vec
                .flatten()
                .collect(),
        ),
        Stmt::Return(expr) => match &expr {
            Expr::TypedExpr(_, r#type) => match r#type {
                Type::Int => {
                    result.append(&mut generate_code_expr(expr, constant_pool, local_var_pool));
                    result.push(Instruction::ireturn);
                }
                Type::Void => {
                    result.push(Instruction::r#return);
                }
                Type::String => {
                    result.append(&mut generate_code_expr(expr, constant_pool, local_var_pool));
                    result.push(Instruction::areturn);
                }
                Type::Bool => {
                    result.append(&mut generate_code_expr(expr, constant_pool, local_var_pool));
                    result.push(Instruction::ireturn);
                }
                Type::Null => {
                    result.push(Instruction::aconst_null);
                    result.push(Instruction::areturn);
                }
                Type::Char => {
                    result.append(&mut generate_code_expr(expr, constant_pool, local_var_pool));
                    result.push(Instruction::ireturn);
                }
                Type::Class(_) => {
                    result.append(&mut generate_code_expr(expr, constant_pool, local_var_pool));
                    result.push(Instruction::areturn);
                }
                _ => panic!("This should never happen"),
            },
            _ => panic!("This should never happen"),
        },
        Stmt::While(expr, stmt) => {
            // TODO: Test, Bene
            result.append(&mut generate_code_expr(expr, constant_pool, local_var_pool));
            // Generate bytecode for our body
            let mut body = generate_code_stmt(*stmt, dir, constant_pool, local_var_pool);
            result.push(Instruction::reljumpifeq(body.len() as i16));
            result.append(&mut body);
            result.push(Instruction::reljumpifeq(-(body.len() as i16)));
        }
        Stmt::LocalVarDecl(types, name) => {
            let index: u8 = local_var_pool.0.len() as u8 + 1;
            // FIXME: Add the variable name to localvarpool and use the index of the added variable for the istore instruction
            match types {
                Type::Int => result.append(&mut vec![
                    Instruction::bipush(index.clone()),
                    Instruction::istore(index),
                ]),
                Type::Bool => result.append(&mut vec![
                    Instruction::bipush(index.clone()),
                    Instruction::istore(index),
                ]),
                Type::Char => result.append(&mut vec![
                    Instruction::bipush(index.clone()),
                    Instruction::istore(index),
                ]),
                Type::String => result.append(&mut vec![
                    Instruction::bipush(index.clone()),
                    Instruction::astore(index),
                ]),
                Type::Null => result.append(&mut vec![
                    Instruction::bipush(index.clone()),
                    Instruction::astore(index),
                ]),
                _ => panic!("Invalid return type"),
            }
            local_var_pool.0.push(name);
        }
        Stmt::If(expr, stmt1, stmt2) => {
            // Generate bytecode for if
            // TODO: Bene, testing
            // Evaluate the expression
            result.append(&mut generate_code_expr(expr, constant_pool, local_var_pool));
            // We set a label to jump to if the expression is false
            let mut if_stmt = generate_code_stmt(*stmt1, dir, constant_pool, local_var_pool);
            // If the expression is false, jump to the else block
            result.push(Instruction::reljumpifeq(if_stmt.len() as i16));
            // If the expression is true, execute the if block
            result.append(&mut if_stmt);
            // If there is an else block, execute it
            if let Some(stmt) = stmt2 {
                result.append(&mut generate_code_stmt(
                    *stmt,
                    dir,
                    constant_pool,
                    local_var_pool,
                ));
            }
        }
        Stmt::StmtExprStmt(stmt_expr) => {
            result.append(&mut generate_code_stmt_expr(
                &stmt_expr,
                constant_pool,
                local_var_pool,
            ));
        }
        Stmt::TypedStmt(stmt, _types) => {
            // Generate bytecode for typed stmt
            // TODO: Check whether we can actually generate the same code as a normal stmt
            result.append(&mut generate_code_stmt(
                *stmt,
                &dir,
                constant_pool,
                local_var_pool,
            ));
        }
    }
    return result;
}

fn generate_code_stmt_expr(
    stmt_expr: &StmtExpr,
    constant_pool: &mut ConstantPool,
    local_var_pool: &mut LocalVarPool,
) -> Vec<Instruction> {
    let mut result = vec![];
    match stmt_expr {
        StmtExpr::Assign(name, expr) => {
            // Generate bytecode for assignment
            result.append(&mut generate_code_expr(
                expr.clone(),
                constant_pool,
                local_var_pool,
            ));
            local_var_pool.add(name.clone());
        }
        StmtExpr::New(types, exprs) => {
            // Generate bytecode for new
            constant_pool.add(Constant::Class(types.to_ir_string().to_string()));
            exprs.iter().for_each(|expr| {
                result.append(&mut generate_code_expr(
                    expr.clone(),
                    constant_pool,
                    local_var_pool,
                ));
            });
        }
        StmtExpr::MethodCall(expr, name, exprs) => {
            // Generate bytecode for method call
            // TODO: Bene
        }
        StmtExpr::TypedStmtExpr(stmt_expr, types) => {
            result.append(&mut generate_code_stmt_expr(
                stmt_expr,
                constant_pool,
                local_var_pool,
            ));
        }
    }
    result
}

fn generate_code_expr(
    expr: Expr,
    constant_pool: &mut ConstantPool,
    local_var_pool: &mut LocalVarPool,
) -> Vec<Instruction> {
    let mut result = vec![];
    // TODO
    match expr {
        Expr::Integer(i) => {
            result.push(Instruction::bipush(i as u8));
        }
        Expr::Bool(b) => {
            result.push(Instruction::bipush(b as u8));
        }
        Expr::Char(c) => {
            result.push(Instruction::bipush(c as u8));
        }
        Expr::String(s) => {
            let ind = constant_pool.add(Constant::Utf8(s.to_string()));
            let index = constant_pool.add(Constant::String(ind));
            result.push(Instruction::ldc(index));
        }
        Expr::Jnull => {
            result.push(Instruction::aconst_null);
        }
        Expr::This => result.push(Instruction::aload(0)),
        Expr::InstVar(exprs, name) => {
            panic!("This should not happen")
        }
        Binary(op, left, right) => match BinaryOp::from(&op as &str) {
            BinaryOp::Add => {
                result.append(&mut generate_code_expr(
                    *left,
                    constant_pool,
                    local_var_pool,
                ));
                result.append(&mut generate_code_expr(
                    *right,
                    constant_pool,
                    local_var_pool,
                ));
                result.push(Instruction::iadd);
            }
            BinaryOp::Sub => {
                result.append(&mut generate_code_expr(
                    *left,
                    constant_pool,
                    local_var_pool,
                ));
                result.append(&mut generate_code_expr(
                    *right,
                    constant_pool,
                    local_var_pool,
                ));
                result.push(Instruction::isub);
            }
            BinaryOp::Mul => {
                result.append(&mut generate_code_expr(
                    *left,
                    constant_pool,
                    local_var_pool,
                ));
                result.append(&mut generate_code_expr(
                    *right,
                    constant_pool,
                    local_var_pool,
                ));
                result.push(Instruction::imul);
            }
            BinaryOp::Div => {
                result.append(&mut generate_code_expr(
                    *left,
                    constant_pool,
                    local_var_pool,
                ));
                result.append(&mut generate_code_expr(
                    *right,
                    constant_pool,
                    local_var_pool,
                ));
                result.push(Instruction::idiv);
            }
            BinaryOp::Mod => {
                result.append(&mut generate_code_expr(
                    *left,
                    constant_pool,
                    local_var_pool,
                ));
                result.append(&mut generate_code_expr(
                    *right,
                    constant_pool,
                    local_var_pool,
                ));
                result.push(Instruction::irem);
            }
            BinaryOp::And => {
                result.append(&mut generate_code_expr(
                    *left,
                    constant_pool,
                    local_var_pool,
                ));
                result.push(Instruction::reljumpifeq(3));
                result.push(Instruction::bipush(1));
                result.push(Instruction::relgoto(2));
                result.push(Instruction::bipush(0));
                result.append(&mut generate_code_expr(
                    *right,
                    constant_pool,
                    local_var_pool,
                ));
                result.push(Instruction::reljumpifeq(3));
                result.push(Instruction::bipush(1));
                result.push(Instruction::relgoto(2));
                result.push(Instruction::bipush(0));
            }
            BinaryOp::Or => {
                result.append(&mut generate_code_expr(
                    *left,
                    constant_pool,
                    local_var_pool,
                ));
                result.push(Instruction::reljumpifne(3));
                result.push(Instruction::bipush(0));
                result.push(Instruction::relgoto(2));
                result.push(Instruction::bipush(1));
                result.append(&mut generate_code_expr(
                    *right,
                    constant_pool,
                    local_var_pool,
                ));
                result.push(Instruction::reljumpifne(3));
                result.push(Instruction::bipush(0));
                result.push(Instruction::relgoto(2));
                result.push(Instruction::bipush(1));
            }
            BinaryOp::Le => {
                result.append(&mut generate_code_expr(
                    *left,
                    constant_pool,
                    local_var_pool,
                ));
                result.append(&mut generate_code_expr(
                    *right,
                    constant_pool,
                    local_var_pool,
                ));
                result.push(Instruction::reljumpiflt(3));
                result.push(Instruction::bipush(1));
                result.push(Instruction::relgoto(2));
                result.push(Instruction::bipush(0));
            }
            BinaryOp::Ge => {
                result.append(&mut generate_code_expr(
                    *left,
                    constant_pool,
                    local_var_pool,
                ));
                result.append(&mut generate_code_expr(
                    *right,
                    constant_pool,
                    local_var_pool,
                ));
                result.push(Instruction::reljumpifge(3));
                result.push(Instruction::bipush(1));
                result.push(Instruction::relgoto(2));
                result.push(Instruction::bipush(0));
            }
            BinaryOp::Lt => {
                result.append(&mut generate_code_expr(
                    *left,
                    constant_pool,
                    local_var_pool,
                ));
                result.append(&mut generate_code_expr(
                    *right,
                    constant_pool,
                    local_var_pool,
                ));
                result.push(Instruction::reljumpifge(3));
                result.push(Instruction::bipush(1));
                result.push(Instruction::relgoto(2));
                result.push(Instruction::bipush(0));
            }
            BinaryOp::Gt => {
                result.append(&mut generate_code_expr(
                    *left,
                    constant_pool,
                    local_var_pool,
                ));
                result.append(&mut generate_code_expr(
                    *right,
                    constant_pool,
                    local_var_pool,
                ));
                result.push(Instruction::reljumpiflt(3));
                result.push(Instruction::bipush(1));
                result.push(Instruction::relgoto(2));
                result.push(Instruction::bipush(0));
            }
            BinaryOp::Eq => {
                result.append(&mut generate_code_expr(
                    *left,
                    constant_pool,
                    local_var_pool,
                ));
                result.append(&mut generate_code_expr(
                    *right,
                    constant_pool,
                    local_var_pool,
                ));
                result.push(Instruction::reljumpifne(3));
                result.push(Instruction::bipush(1));
                result.push(Instruction::relgoto(2));
                result.push(Instruction::bipush(0));
            }
            BinaryOp::Ne => {
                result.append(&mut generate_code_expr(
                    *left,
                    constant_pool,
                    local_var_pool,
                ));
                result.append(&mut generate_code_expr(
                    *right,
                    constant_pool,
                    local_var_pool,
                ));
                result.push(Instruction::reljumpifeq(3));
                result.push(Instruction::bipush(1));
                result.push(Instruction::relgoto(2));
                result.push(Instruction::bipush(0))
            }
        },
        Expr::Unary(op, expr) => {
            result.append(&mut generate_code_expr(
                *expr,
                constant_pool,
                local_var_pool,
            ));
            match UnaryOp::from(&op as &str) {
                UnaryOp::Not => {
                    result.push(Instruction::reljumpifne(3));
                    result.push(Instruction::bipush(1));
                    result.push(Instruction::relgoto(2));
                    result.push(Instruction::bipush(0));
                }
                UnaryOp::Neg => {
                    result.push(Instruction::ineg);
                }
                UnaryOp::Pos => {}
            }
        }
        Expr::LocalVar(name) => {
            //Todo: Match for type
            let index = local_var_pool.get_index(&name);
            result.push(Instruction::iload(index as u8));
        }
        Expr::TypedExpr(expr, r#type) => {
            result.append(&mut generate_code_expr(
                *expr,
                constant_pool,
                local_var_pool,
            ));
        }
        Expr::StmtExprExpr(stmt_expr) => {
            result.append(&mut generate_code_stmt_expr(
                &stmt_expr,
                constant_pool,
                local_var_pool,
            ));
        }
        Expr::FieldVar(name) => {
            //TODO: How to get class name and type?
            constant_pool.add(Constant::FieldRef(FieldRef {
                class: "0".to_string(),
                field: NameAndType {
                    name: name.clone(),
                    r#type: "Int".to_string(),
                },
            }));
            //Todo: Write Fieldvar as Fieldref into Constantpool
        }
        unexpected => panic!("Unexpected expression: {:?}", unexpected),
    }
    result
}
//TODO: Constructor
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_fields() {
        let mut constant_pool = ConstantPool::new();
        let field = FieldDecl {
            field_type: Type::Int,
            name: String::from("test"),
            val: None,
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
