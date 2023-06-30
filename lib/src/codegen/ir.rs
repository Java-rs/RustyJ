#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(non_snake_case)]

use crate::codegen::reljumps::convert_to_absolute_jumps;
use crate::codegen::Instruction::getfield;
use crate::types::Expr::Binary;
use crate::types::*;
use std::fmt::Debug;
use std::ops::Deref;

static JAVA_LANG_OBJECT: &str = "java/lang/Object";
static OBJECT_INIT_METHOD: &str = "<init>";
static OBJECT_INIT_RET: &str = "()V";

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
    // We also assume that the constant pool has already been filled completely
    pub fn as_bytes(&mut self) -> Vec<u8> {
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
            .index_of(&Constant::Class(current_class.name.clone()))
            .unwrap();
        let super_class_index = self
            .constant_pool
            .index_of(&Constant::Class(JAVA_LANG_OBJECT.to_string()))
            .unwrap();
        let mut field_infos = current_class
            .fields
            .iter()
            .map(|f| f.as_bytes(&current_class.name, &mut self.constant_pool))
            .flatten()
            .collect();
        let mut method_infos = current_class
            .methods
            .iter()
            .map(|m| m.as_bytes(&mut self.constant_pool))
            .flatten()
            .collect();

        // Constant Pool
        result.extend_from_slice(&self.constant_pool.count().to_be_bytes());
        result.append(&mut self.constant_pool.as_bytes());
        // Access flags; 0x01 = public, 0x20 = super where superclass-methods are treated specially
        result.extend_from_slice(&[0, 32]);
        result.extend_from_slice(&this_class_index.to_be_bytes());
        result.extend_from_slice(&super_class_index.to_be_bytes());
        result.extend_from_slice(&[0, 0]); // Interfaces count, being 0

        // Fields
        result.extend_from_slice(&(current_class.fields.len() as u16).to_be_bytes());
        result.append(&mut field_infos);

        // Methods
        // @Cleanup +1 because of the default constructor
        result.extend_from_slice(&(current_class.methods.len() as u16 + 1).to_be_bytes());
        result.append(&mut method_infos);

        result.extend_from_slice(&[0, 0]);
        result.extend_from_slice(&[]);
        result
    }
}

fn make_default_constructor(class: &IRClass, constant_pool: &mut ConstantPool) -> CompiledMethod {
    let mut code = vec![
        Instruction::aload_0,
        Instruction::invokespecial(
            constant_pool
                .index_of(&Constant::MethodRef(MethodRef {
                    class: JAVA_LANG_OBJECT.to_string(),
                    method: NameAndType {
                        name: OBJECT_INIT_METHOD.to_string(),
                        r#type: OBJECT_INIT_RET.to_string(),
                    },
                }))
                .unwrap(),
        ),
    ];
    let mut local_var_pool = LocalVarPool(vec![]);
    let mut stack = StackSize::new();
    for field in class.fields.iter() {
        if let Some(x) = &field.val {
            code.push(Instruction::aload_0);
            code.append(&mut generate_code_expr(
                Expr::TypedExpr(Box::new(x.clone()), field.field_type.clone()),
                &mut stack,
                constant_pool,
                &mut local_var_pool,
                &class.name,
            ));
            code.push(Instruction::putfield(constant_pool.add(
                Constant::FieldRef(FieldRef {
                    class: class.name.clone(),
                    field: NameAndType {
                        name: field.name.clone(),
                        r#type: field.field_type.to_ir_string(),
                    },
                }),
            )))
        }
    }
    code.push(Instruction::r#return);
    CompiledMethod {
        name: "<init>".to_string(),
        return_type: Type::Void,
        params: vec![],
        max_stack: stack.max,
        max_locals: 1 + local_var_pool.0.len() as u16,
        code,
    }
}

#[derive(Debug)]
pub struct ConstantPool(Vec<Constant>);
impl ConstantPool {
    fn new(name: &str) -> Self {
        // This is the same boilerplate constantpool for all files
        // so we can just hardcode it here.
        // This would only ever change, if we allowed the user
        // to create their own constructors, which we don't
        Self(vec![
            Constant::MethodRef(MethodRef {
                class: JAVA_LANG_OBJECT.to_string(),
                method: NameAndType {
                    name: OBJECT_INIT_METHOD.to_string(),
                    r#type: OBJECT_INIT_RET.to_string(),
                },
            }),
            Constant::Class(JAVA_LANG_OBJECT.to_string()),
            Constant::NameAndType(NameAndType {
                name: OBJECT_INIT_METHOD.to_string(),
                r#type: OBJECT_INIT_RET.to_string(),
            }),
            Constant::Utf8(JAVA_LANG_OBJECT.to_string()),
            Constant::Utf8(OBJECT_INIT_METHOD.to_string()),
            Constant::Utf8(OBJECT_INIT_RET.to_string()),
            Constant::Class(name.to_string()),
            Constant::Utf8(name.to_string()),
            Constant::Utf8("Code".to_string()),
        ])
    }
    // For some unknown reason, this is 1-indexed and we have to add 1 to the count
    fn count(&self) -> u16 {
        self.0.len() as u16 + 1
    }
    /// Adds a constant to the constant pool, returning its index
    pub fn add(&mut self, constant: Constant) -> u16 {
        if let Some(index) = self.index_of(&constant) {
            return index;
        }
        match constant.clone() {
            Constant::Class(class_name) => {
                self.add(Constant::Utf8(class_name));
            }
            Constant::FieldRef(field_ref) => {
                self.add(Constant::Utf8(field_ref.class));
                self.add(Constant::NameAndType(field_ref.field));
            }
            Constant::NameAndType(name_and_type) => {
                self.add(Constant::Utf8(name_and_type.name));
                self.add(Constant::Utf8(name_and_type.r#type));
            }
            Constant::MethodRef(method_ref) => {
                self.add(Constant::Utf8(method_ref.class));
                self.add(Constant::NameAndType(method_ref.method));
            }
            // Do nothing in these cases
            Constant::String(str_idx) => {}
            Constant::Utf8(name) => {}
        };
        self.0.push(constant);
        self.0.len() as u16
    }
    fn index_of(&self, constant: &Constant) -> Option<u16> {
        self.0
            .iter()
            .position(|c| *c == *constant)
            .and_then(|x| Some(x as u16 + 1)) // +1 because the constant pool is 1-indexed
    }
    /// Returns the constant at the given index. Note that this is 1-indexed since the constant
    /// pool of the JVM is 1-indexed
    fn get(&self, index: u16) -> Option<&Constant> {
        self.0.get(index as usize - 1)
    }
    /// See this table: https://docs.oracle.com/javase/specs/jvms/se15/html/jvms-4.html#jvms-4.4
    fn as_bytes(&mut self) -> Vec<u8> {
        let mut result = vec![];
        for idx in 0..self.0.len() {
            let constant = self.0.get(idx).unwrap().clone();
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
                        &self
                            .index_of(&Constant::Utf8(name.clone()))
                            .unwrap()
                            .to_be_bytes(),
                    );
                }
                Constant::MethodRef(MethodRef { class, method }) => {
                    result.push(10);
                    result.extend_from_slice(
                        &self
                            .index_of(&Constant::Class(class))
                            .unwrap()
                            .to_be_bytes(),
                    );
                    result.extend_from_slice(
                        &self
                            .index_of(&Constant::NameAndType(method))
                            .unwrap()
                            .to_be_bytes(),
                    );
                }
                Constant::NameAndType(NameAndType { name, r#type }) => {
                    result.push(12);
                    result.extend_from_slice(
                        &self.index_of(&Constant::Utf8(name)).unwrap().to_be_bytes(),
                    );
                    result.extend_from_slice(&self.add(Constant::Utf8(r#type)).to_be_bytes());
                }
                Constant::FieldRef(FieldRef { class, field }) => {
                    //TODO: Maybe this should be moved Prio2
                    result.push(9);
                    result.extend_from_slice(
                        &self
                            .index_of(&Constant::Class(class.clone()))
                            .unwrap()
                            .to_be_bytes(),
                    );
                    result.extend_from_slice(
                        &self
                            .index_of(&Constant::NameAndType(field.clone()))
                            .unwrap()
                            .to_be_bytes(),
                    );
                }
                Constant::String(val) => {
                    result.push(8);
                    result.extend_from_slice(
                        &self
                            .index_of(&Constant::String(val.clone()))
                            .unwrap()
                            .to_be_bytes(),
                    );
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
    pub fn add(&mut self, name: String) -> u8 {
        // println!("Adding local var {:?}", name);
        self.0.push(name);
        self.0.len() as u8
    }
    pub fn get_index(&self, name: &str) -> u8 {
        self.0
            .iter()
            .position(|n| n == name)
            .map(|i| i as u8)
            .expect(&*format!("Local var {:?} not found in  {:?}", name, self.0))
    }
}
#[derive(Debug)]
pub(crate) struct CompiledMethod {
    pub(crate) name: String,
    pub(crate) return_type: Type,
    pub(crate) params: Vec<(Type, String)>,
    pub(crate) max_stack: u16,
    pub(crate) max_locals: u16,
    pub(crate) code: Vec<Instruction>,
}

impl CompiledMethod {
    /// Get the method info as raw bytes as described in https://docs.oracle.com/javase/specs/jvms/se15/html/jvms-4.html#jvms-4.6
    fn as_bytes(&self, constant_pool: &mut ConstantPool) -> Vec<u8> {
        let mut result = vec![];
        // Access flags, since we don't support access flags, they are always 0
        result.extend_from_slice(&[0, 0]);
        // Name index
        result.extend_from_slice(
            &constant_pool
                .add(Constant::Utf8(self.name.clone()))
                .to_be_bytes(),
        );
        // Descriptor index. ()V for void, ()I for int and ()Z for bool because java developers are insane
        result.extend_from_slice(
            &constant_pool
                .add(Constant::Utf8(format!(
                    "({}){}",
                    self.params
                        .iter()
                        .map(|p| p.0.to_ir_string())
                        .collect::<Vec<String>>()
                        .join(""),
                    self.return_type.to_ir_string()
                )))
                .to_be_bytes(),
        );
        // Attributes:
        // For methods we only create the Code-Attribute at the moment
        // attributes_count = 1 because we only have the Code-Attribute
        result.extend_from_slice(&[0, 1]);
        // Name Index
        result.extend_from_slice(
            &constant_pool
                .index_of(&Constant::Utf8("Code".to_string()))
                .unwrap()
                .to_be_bytes(),
        );
        // Expand the relative jumps in the code to absolute jumps
        let expanded_code = convert_to_absolute_jumps(self.code.clone());
        // attr = attribute after attribute_length
        let mut attr = vec![];
        attr.extend_from_slice(&self.max_stack.to_be_bytes());
        attr.extend_from_slice(&self.max_locals.to_be_bytes());
        let mut code_bytes = vec![];
        expanded_code
            .iter()
            .for_each(|i| code_bytes.append(&mut i.as_bytes()));
        attr.extend_from_slice(&(code_bytes.len() as u32).to_be_bytes());
        attr.append(&mut code_bytes);
        attr.extend_from_slice(&[0, 0]); // Exception table length
        attr.extend_from_slice(&[0, 0]); // Inner Attributes count

        // Attribute length
        result.extend_from_slice(&(attr.len() as u32).to_be_bytes());
        result.append(&mut attr);
        result
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Constant {
    Class(String),
    /// This has to be of format `class_index.name_and_type_index`
    FieldRef(FieldRef),
    /// This has to be of format `class_index.method_name_index`. If it is later found to be beneficial however we could split this into two Strings
    MethodRef(MethodRef),
    NameAndType(NameAndType),
    String(u16),
    Utf8(String),
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FieldRef {
    pub class: String,
    pub field: NameAndType,
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MethodRef {
    class: String,
    method: NameAndType,
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct NameAndType {
    pub name: String,
    pub r#type: String,
}

/// The instructions for the JVM
/// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-6.html#jvms-6.5.areturn
#[derive(Debug, Copy, Clone)]
pub(crate) enum Instruction {
    invokespecial(u16), //Calling a method from the super class (probably only used in constructor)
    aload_0,
    aload(u8),        //Load reference from local variable
    iload(u8),        //Load int from local variable
    ifeq(u16),        //Branch if int is 0
    iflt(u16),        //Branch if int is < 0
    ifge(u16),        //Branch if int is >= 0
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
    putfield(u16), //Sets a value for the field at the given index. The stack must have the reference to the object to which the field belongs and on top of that the value to set the field to
    getfield(u16), // Get field from object via an index into the constant pool
    new(u16),      //Create new object
    dup,           //Duplicate the top value on the stack
}

fn high_byte(short: u16) -> u8 {
    (short >> 8) as u8
}

fn low_byte(short: u16) -> u8 {
    short as u8
}

impl Instruction {
    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        match self {
            Instruction::invokespecial(idx) => {
                vec![183, high_byte(*idx), low_byte(*idx)]
            }
            Instruction::aload_0 => vec![42],
            Instruction::aload(idx) => vec![25, *idx],
            Instruction::iload(idx) => vec![21, *idx],
            Instruction::ifeq(jmp) => vec![153, high_byte(*jmp), low_byte(*jmp)],
            Instruction::ireturn => vec![172],
            Instruction::r#return => vec![177],
            Instruction::areturn => vec![176],
            Instruction::bipush(byte) => vec![16, *byte],
            Instruction::istore(idx) => vec![54, *idx],
            Instruction::astore(idx) => vec![58, *idx],
            Instruction::aconst_null => vec![1],
            Instruction::ldc(idx) => vec![18, high_byte(*idx), low_byte(*idx)],
            Instruction::ineg => vec![116],
            Instruction::goto(jmp) => vec![167, high_byte(*jmp), low_byte(*jmp)],
            Instruction::ifne(jmp) => vec![154, high_byte(*jmp), low_byte(*jmp)],
            Instruction::ifge(jmp) => vec![156, high_byte(*jmp), low_byte(*jmp)],
            Instruction::iflt(jmp) => vec![155, high_byte(*jmp), low_byte(*jmp)],
            Instruction::iadd => vec![96],
            Instruction::isub => vec![100],
            Instruction::imul => vec![104],
            Instruction::idiv => vec![108],
            Instruction::irem => vec![112],
            Instruction::putfield(idx) => vec![181, high_byte(*idx), low_byte(*idx)],
            Instruction::getfield(idx) => vec![180, high_byte(*idx), low_byte(*idx)],
            Instruction::new(idx) => vec![187, high_byte(*idx), low_byte(*idx)],
            Instruction::dup => vec![89],
            // Instruction::relgoto() =>
            // Instruction::reljumpifeq(idx) =>
            // Instruction::reljumpifne(idx) =>
            e => panic!("Instruction {:?} not implemented or unexpected", e),
        }
    }
}

pub fn generate_dir(ast: &Prg) -> DIR {
    let mut dir = DIR {
        constant_pool: ConstantPool::new(&ast.get(0).unwrap().name),
        classes: vec![],
    };
    for class in ast {
        let ir_class = generate_class(class, &mut dir);
        dir.classes.push(ir_class);
    }
    dir
}

fn generate_class(class: &Class, dir: &mut DIR) -> IRClass {
    let mut ir_class = IRClass::new(class.name.clone(), vec![], vec![]);
    for field in &class.fields {
        ir_class.fields.push(field.clone());
    }
    ir_class
        .methods
        .push(make_default_constructor(&ir_class, &mut dir.constant_pool));
    for method in &class.methods {
        ir_class
            .methods
            .push(generate_method(method, &mut dir.constant_pool, &class.name));
    }
    ir_class
}
/// If this method is used the caller has to still set a NameAndType constant and a FieldRef
/// constant, which is technically optional if the field is not used but we're lazy
fn generate_field(field: &FieldDecl, constant_pool: &mut ConstantPool) -> IRFieldDecl {
    let name_index = constant_pool.add(Constant::Utf8(field.name.clone()));
    let type_index = constant_pool.add(Constant::Utf8(format!(
        "(){}",
        field.field_type.clone().to_ir_string()
    )));
    IRFieldDecl::new(type_index, name_index)
}

/// Generates a Vector of instructions for a given method
fn generate_method(
    method: &MethodDecl,
    constant_pool: &mut ConstantPool,
    class_name: &str,
) -> CompiledMethod {
    let mut local_var_pool = LocalVarPool(
        method
            .params
            .iter()
            .map(|(_type, name)| name.clone())
            .collect(),
    );
    let mut stack = StackSize::new();
    let code = generate_code_stmt(
        method.body.clone(),
        &mut stack,
        constant_pool,
        &mut local_var_pool,
        class_name,
    );
    CompiledMethod {
        name: method.name.clone(),
        return_type: method.ret_type.clone(),
        params: method.params.clone(),
        max_stack: stack.max,
        max_locals: 1 + local_var_pool.0.len() as u16,
        code,
    }
}

#[derive(Debug)]
struct StackSize {
    current: u16,
    max: u16,
}

impl StackSize {
    pub fn new() -> Self {
        StackSize { current: 0, max: 0 }
    }

    pub fn update(&mut self, step: i16) {
        self.current = (self.current as i16 + step) as u16;
        if self.current > self.max {
            self.max = self.current;
        }
    }

    pub fn set(&mut self, val: u16) {
        self.current = val;
        if self.current > self.max {
            self.max = self.current;
        }
    }
}

fn generate_code_stmt(
    stmt: Stmt,
    stack: &mut StackSize,
    constant_pool: &mut ConstantPool,
    local_var_pool: &mut LocalVarPool,
    class_name: &str,
) -> Vec<Instruction> {
    let mut result = vec![];
    match stmt {
        Stmt::TypedStmt(stmt, stmt_type) => {
            // Generate bytecode for typed stmt
            let stmt = stmt.deref().clone();
            match stmt {
                Stmt::Block(stmts) => {
                    for stmt in stmts {
                        result.append(&mut generate_code_stmt(
                            stmt.clone(),
                            stack,
                            constant_pool,
                            local_var_pool,
                            class_name,
                        ));
                    }
                }
                Stmt::Return(expr) => {
                    match &expr {
                        Expr::TypedExpr(_, r#type) => match r#type {
                            Type::Int => {
                                result.append(&mut generate_code_expr(
                                    expr,
                                    stack,
                                    constant_pool,
                                    local_var_pool,
                                    class_name,
                                ));
                                result.push(Instruction::ireturn);
                            }
                            Type::Void => {
                                result.push(Instruction::r#return);
                            }
                            Type::String => {
                                result.append(&mut generate_code_expr(
                                    expr,
                                    stack,
                                    constant_pool,
                                    local_var_pool,
                                    class_name,
                                ));
                                result.push(Instruction::areturn);
                            }
                            Type::Bool => {
                                result.append(&mut generate_code_expr(
                                    expr,
                                    stack,
                                    constant_pool,
                                    local_var_pool,
                                    class_name,
                                ));
                                result.push(Instruction::ireturn);
                            }
                            Type::Null => {
                                result.push(Instruction::aconst_null);
                                result.push(Instruction::areturn);
                            }
                            Type::Char => {
                                result.append(&mut generate_code_expr(
                                    expr,
                                    stack,
                                    constant_pool,
                                    local_var_pool,
                                    class_name,
                                ));
                                result.push(Instruction::ireturn);
                            }
                            Type::Class(_) => {
                                result.append(&mut generate_code_expr(
                                    expr,
                                    stack,
                                    constant_pool,
                                    local_var_pool,
                                    class_name,
                                ));
                                result.push(Instruction::areturn);
                            }
                            _ => panic!("This should never happen"),
                        },
                        _ => panic!("This should never happen"),
                    };
                    stack.set(0);
                }
                Stmt::While(expr, stmt) => {
                    // Generate bytecode for our condition
                    result.append(&mut generate_code_expr(
                        expr,
                        stack,
                        constant_pool,
                        local_var_pool,
                        class_name,
                    ));
                    // Checking the condition removes one element from stack
                    stack.update(-1);
                    // Generate bytecode for our body
                    let mut body =
                        generate_code_stmt(*stmt, stack, constant_pool, local_var_pool, class_name);
                    result.push(Instruction::reljumpifeq(body.len() as i16));
                    result.append(&mut body);
                    result.push(Instruction::reljumpifeq(-(body.len() as i16)));
                }
                Stmt::LocalVarDecl(types, name) => {
                    local_var_pool.add(name.clone());
                    stack.update(1);
                }
                Stmt::If(expr, stmt1, stmt2) => {
                    // Generate bytecode for if
                    // Evaluate the condition
                    result.append(&mut generate_code_expr(
                        expr,
                        stack,
                        constant_pool,
                        local_var_pool,
                        class_name,
                    ));
                    // Checking the condition removes an element from the stack
                    stack.update(-1);
                    // We set a label to jump to if the expression is false
                    let mut if_body = generate_code_stmt(
                        *stmt1,
                        stack,
                        constant_pool,
                        local_var_pool,
                        class_name,
                    );
                    // If the expression is false, jump to the else block
                    result.push(Instruction::reljumpifeq(if_body.len() as i16));
                    // If the expression is true, execute the if block
                    result.append(&mut if_body);
                    // If there is an else block, execute it
                    if let Some(stmt) = stmt2 {
                        let mut else_body = generate_code_stmt(
                            *stmt,
                            stack,
                            constant_pool,
                            local_var_pool,
                            class_name,
                        );
                        result.append(&mut else_body);
                    }
                }
                Stmt::StmtExprStmt(stmt_expr) => {
                    result.append(&mut generate_code_stmt_expr(
                        &stmt_expr,
                        stack,
                        constant_pool,
                        local_var_pool,
                        class_name,
                    ));
                }
                Stmt::TypedStmt(_, _) => panic!("Expected untyped statement, got typed statement"),
            }
        }
        statement => panic!("Expected Typed Statement, got {:?}", statement),
    };
    result
}

fn generate_code_stmt_expr(
    stmt_expr: &StmtExpr,
    stack: &mut StackSize,
    constant_pool: &mut ConstantPool,
    local_var_pool: &mut LocalVarPool,
    class_name: &str,
) -> Vec<Instruction> {
    let mut result = vec![];
    match stmt_expr {
        StmtExpr::TypedStmtExpr(new_stmt_expr, expr_type) => {
            match new_stmt_expr.deref() {
                StmtExpr::Assign(var, expr) => {
                    // Generate bytecode for assignment
                    let mut expr_code = generate_code_expr(
                        expr.clone(),
                        stack,
                        constant_pool,
                        local_var_pool,
                        class_name,
                    );
                    match var {
                        Expr::TypedExpr(var, t) => match var.deref() {
                            Expr::LocalVar(name) => {
                                let idx = local_var_pool.get_index(name);
                                result.append(&mut expr_code);
                                if let Type::Class(n) = t {
                                    assert_eq!(n, name);
                                    result.push(Instruction::astore(idx));
                                } else if let Type::String = t {
                                    result.push(Instruction::astore(idx));
                                } else {
                                    result.push(Instruction::istore(idx));
                                }
                                stack.update(-1);
                            }
                            Expr::FieldVar(name) => {
                                let idx = constant_pool.add(Constant::FieldRef(FieldRef {
                                    class: class_name.to_string(),
                                    field: NameAndType {
                                        name: name.to_string(),
                                        r#type: t.to_ir_string(),
                                    },
                                }));
                                result.push(Instruction::aload_0);
                                stack.update(1);
                                result.append(&mut expr_code);
                                result.push(Instruction::putfield(idx));
                                stack.update(-2);
                            }
                            Expr::InstVar(expr, name) => {
                                let idx = constant_pool.add(Constant::FieldRef(FieldRef {
                                    class: class_name.to_string(),
                                    field: NameAndType {
                                        name: name.to_string(),
                                        r#type: t.to_ir_string(),
                                    },
                                }));
                                result.append(&mut generate_code_expr(
                                    expr.deref().clone(),
                                    stack,
                                    constant_pool,
                                    local_var_pool,
                                    class_name,
                                ));
                                result.append(&mut expr_code);
                                result.push(Instruction::putfield(idx));
                                stack.update(-2);
                            }
                            _ => panic!("Unexpected variable type for assignment: {:?}", var),
                        },
                        _ => panic!("Expected typed stmt"),
                    }
                }
                StmtExpr::New(types, exprs) => {
                    // Generate bytecode for new
                    let class_index =
                        constant_pool.add(Constant::Class(types.to_ir_string().to_string()));
                    let method_index = constant_pool.add(Constant::MethodRef(MethodRef {
                        class: types.to_ir_string(),
                        method: NameAndType {
                            name: "<init>".to_string(),
                            r#type: "()V".to_string(),
                        },
                    }));
                    result.push(Instruction::new(class_index));
                    result.push(Instruction::dup);
                    result.push(Instruction::invokespecial(method_index));
                    stack.update(1);
                    stack.update(-1);
                }
                StmtExpr::MethodCall(_expr, name, args) => {
                    // Generate bytecode for method call
                    // Principally this should work this way:
                    // 1. Write Function Name into Constant Pool generating the necessary Constants
                    // 2. Push all arguments onto the stack
                    // 3. Call invokespecial on the given back function index
                    result.append(
                        &mut args
                            .iter()
                            .flat_map(|arg| {
                                generate_code_expr(
                                    arg.clone(),
                                    stack,
                                    constant_pool,
                                    local_var_pool,
                                    class_name,
                                )
                            })
                            .collect(),
                    );
                    fn generate_name_and_type(return_type: &Type, args: &Vec<Expr>) -> String {
                        // Argument types comma seperated
                        let argument_types = args
                            .iter()
                            .map(|arg| {
                                arg.get_type()
                                    .expect("Expected typed argument")
                                    .to_ir_string()
                                    + ","
                            })
                            .collect::<String>();
                        let argument_types = if argument_types.is_empty() {
                            &argument_types
                        } else {
                            &argument_types[..argument_types.len() - 1]
                        };
                        format!(
                            "{}:({}){}",
                            return_type.to_ir_string(),
                            argument_types,
                            return_type.to_ir_string()
                        )
                    }
                    let method_index = constant_pool.add(Constant::MethodRef(MethodRef {
                        class: class_name.to_string(),
                        method: NameAndType {
                            name: name.clone(),
                            r#type: generate_name_and_type(expr_type, args),
                        },
                    }));
                    result.push(Instruction::invokespecial(method_index));
                    stack.update(1);
                }
                _ => panic!("StmtExpr typed: {:?}", new_stmt_expr),
            }
        }
        e => panic!("StmtExpr not typed: {:?}", e),
    }
    result
}

fn generate_code_expr(
    expr: Expr,
    stack: &mut StackSize,
    constant_pool: &mut ConstantPool,
    local_var_pool: &mut LocalVarPool,
    class_name: &str,
) -> Vec<Instruction> {
    let mut result = vec![];
    match expr {
        Expr::TypedExpr(expr, r#type) => {
            let expr = expr.deref().clone();
            match expr {
                Expr::Integer(i) => {
                    result.push(Instruction::bipush(i as u8));
                    stack.update(1);
                }
                Expr::Bool(b) => {
                    result.push(Instruction::bipush(b as u8));
                    stack.update(1);
                }
                Expr::Char(c) => {
                    result.push(Instruction::bipush(c as u8));
                    stack.update(1);
                }
                Expr::String(s) => {
                    let ind = constant_pool.add(Constant::Utf8(s.to_string()));
                    let index = constant_pool.add(Constant::String(ind));
                    result.push(Instruction::ldc(index));
                    stack.update(1);
                }
                Expr::Jnull => {
                    result.push(Instruction::aconst_null);
                    stack.update(1);
                }
                Expr::This => {
                    result.push(Instruction::aload(0));
                    stack.update(1);
                }
                Expr::InstVar(exprs, name) => {
                    match exprs.deref() {
                        Expr::TypedExpr(expr, r#type) => match expr.deref() {
                            Expr::This => {
                                result.push(Instruction::aload(0));
                                result.push(Instruction::getfield(constant_pool.add(
                                    Constant::FieldRef(FieldRef {
                                        class: class_name.to_string(),
                                        field: NameAndType {
                                            name: name.clone(),
                                            r#type: r#type.to_ir_string(),
                                        },
                                    }),
                                )));
                            }
                            Expr::LocalVar(name) => {
                                let idx = local_var_pool.get_index(name);
                                result.push(Instruction::aload(idx));
                                result.push(Instruction::getfield(constant_pool.add(
                                    Constant::FieldRef(FieldRef {
                                        class: class_name.to_string(),
                                        field: NameAndType {
                                            name: name.clone(),
                                            r#type: r#type.to_ir_string(),
                                        },
                                    }),
                                )));
                            }
                            Expr::FieldVar(name) => {
                                let field_index = constant_pool.add(Constant::FieldRef(FieldRef {
                                    class: class_name.to_string(),
                                    field: NameAndType {
                                        name: name.clone(),
                                        r#type: r#type.to_ir_string(),
                                    },
                                }));
                                result.push(Instruction::getfield(field_index));
                            }
                            _ => panic!("Expected this got {:?}", exprs),
                        },
                        _ => panic!("Expected typed stmt got {:?}", exprs),
                    }
                    let field_index = constant_pool.add(Constant::FieldRef(FieldRef {
                        class: class_name.to_string(),
                        field: NameAndType {
                            name: name.clone(),
                            r#type: r#type.to_ir_string(),
                        },
                    }));
                    result.push(getfield(field_index));
                    // I'm thinking 2 here since we load the field here too and leave the class on the stack
                    stack.update(2);
                }

                Binary(op, left, right) => {
                    stack.update(-1);
                    match BinaryOp::from(&op as &str) {
                        BinaryOp::Add => {
                            result.append(&mut generate_code_expr(
                                *left,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.append(&mut generate_code_expr(
                                *right,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.push(Instruction::iadd);
                        }
                        BinaryOp::Sub => {
                            result.append(&mut generate_code_expr(
                                *left,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.append(&mut generate_code_expr(
                                *right,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.push(Instruction::isub);
                        }
                        BinaryOp::Mul => {
                            result.append(&mut generate_code_expr(
                                *left,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.append(&mut generate_code_expr(
                                *right,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.push(Instruction::imul);
                        }
                        BinaryOp::Div => {
                            result.append(&mut generate_code_expr(
                                *left,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.append(&mut generate_code_expr(
                                *right,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.push(Instruction::idiv);
                        }
                        BinaryOp::Mod => {
                            result.append(&mut generate_code_expr(
                                *left,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.append(&mut generate_code_expr(
                                *right,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.push(Instruction::irem);
                        }
                        BinaryOp::And => {
                            result.append(&mut generate_code_expr(
                                *left,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.push(Instruction::reljumpifeq(3));
                            result.push(Instruction::bipush(1));
                            result.push(Instruction::relgoto(2));
                            result.push(Instruction::bipush(0));
                            result.append(&mut generate_code_expr(
                                *right,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.push(Instruction::reljumpifeq(3));
                            result.push(Instruction::bipush(1));
                            result.push(Instruction::relgoto(2));
                            result.push(Instruction::bipush(0));
                        }
                        BinaryOp::Or => {
                            result.append(&mut generate_code_expr(
                                *left,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.push(Instruction::reljumpifne(3));
                            result.push(Instruction::bipush(0));
                            result.push(Instruction::relgoto(2));
                            result.push(Instruction::bipush(1));
                            result.append(&mut generate_code_expr(
                                *right,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.push(Instruction::reljumpifne(3));
                            result.push(Instruction::bipush(0));
                            result.push(Instruction::relgoto(2));
                            result.push(Instruction::bipush(1));
                        }
                        BinaryOp::Le => {
                            result.append(&mut generate_code_expr(
                                *left,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.append(&mut generate_code_expr(
                                *right,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.push(Instruction::reljumpiflt(3));
                            result.push(Instruction::bipush(1));
                            result.push(Instruction::relgoto(2));
                            result.push(Instruction::bipush(0));
                        }
                        BinaryOp::Ge => {
                            result.append(&mut generate_code_expr(
                                *left,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.append(&mut generate_code_expr(
                                *right,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.push(Instruction::reljumpifge(3));
                            result.push(Instruction::bipush(1));
                            result.push(Instruction::relgoto(2));
                            result.push(Instruction::bipush(0));
                        }
                        BinaryOp::Lt => {
                            result.append(&mut generate_code_expr(
                                *left,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.append(&mut generate_code_expr(
                                *right,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.push(Instruction::reljumpifge(3));
                            result.push(Instruction::bipush(1));
                            result.push(Instruction::relgoto(2));
                            result.push(Instruction::bipush(0));
                        }
                        BinaryOp::Gt => {
                            result.append(&mut generate_code_expr(
                                *left,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.append(&mut generate_code_expr(
                                *right,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.push(Instruction::reljumpiflt(3));
                            result.push(Instruction::bipush(1));
                            result.push(Instruction::relgoto(2));
                            result.push(Instruction::bipush(0));
                        }
                        BinaryOp::Eq => {
                            result.append(&mut generate_code_expr(
                                *left,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.append(&mut generate_code_expr(
                                *right,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.push(Instruction::reljumpifne(3));
                            result.push(Instruction::bipush(1));
                            result.push(Instruction::relgoto(2));
                            result.push(Instruction::bipush(0));
                        }
                        BinaryOp::Ne => {
                            result.append(&mut generate_code_expr(
                                *left,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.append(&mut generate_code_expr(
                                *right,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            ));
                            result.push(Instruction::reljumpifeq(3));
                            result.push(Instruction::bipush(1));
                            result.push(Instruction::relgoto(2));
                            result.push(Instruction::bipush(0))
                        }
                    }
                }
                Expr::Unary(op, expr) => {
                    result.append(&mut generate_code_expr(
                        *expr,
                        stack,
                        constant_pool,
                        local_var_pool,
                        class_name,
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
                    stack.update(1);
                    let index = local_var_pool.get_index(&name);
                    match r#type {
                        Type::Int => {
                            result.push(Instruction::iload(index as u8));
                        }
                        Type::Bool => {
                            result.push(Instruction::iload(index as u8));
                        }
                        Type::Char => {
                            result.push(Instruction::iload(index as u8));
                        }
                        Type::String => {
                            result.push(Instruction::aload(index as u8));
                        }
                        _ => panic!("Unexpected type: {:?}", r#type),
                    }
                }

                Expr::StmtExprExpr(stmt_expr) => {
                    result.append(&mut generate_code_stmt_expr(
                        &stmt_expr,
                        stack,
                        constant_pool,
                        local_var_pool,
                        class_name,
                    ));
                }
                Expr::FieldVar(name) => {
                    let index = constant_pool.add(Constant::FieldRef(FieldRef {
                        class: class_name.to_string(),
                        field: NameAndType {
                            name: name.clone(),
                            r#type: r#type.to_string(),
                        },
                    }));
                    // We only do getfield here because we don't know what operation we're doing
                    // with the field
                    result.push(Instruction::getfield(index));
                    stack.update(1);
                }
                p => panic!(
                    "Unexpected expression where untyped expression was expected: {:?}",
                    p
                ),
            }
        }
        unexpected => panic!("Unexpected expression: {:?}", unexpected),
    }
    result
}
