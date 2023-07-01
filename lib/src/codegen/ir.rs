#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(non_snake_case)]

use super::stack::*;
use super::Instruction::getfield;
use super::*;
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
            .flat_map(|f| f.as_bytes(&current_class.name, &mut self.constant_pool))
            .collect();
        let mut method_infos = current_class
            .methods
            .iter()
            .flat_map(|m| m.as_bytes(&mut self.constant_pool))
            .collect();
        println!("Constant pool: {:?}", self.constant_pool);
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
        result.extend_from_slice(&(current_class.methods.len() as u16).to_be_bytes());
        result.append(&mut method_infos);

        result.extend_from_slice(&[0, 0]);
        result.extend_from_slice(&[]);
        result
    }
}

fn make_default_constructor(class: &IRClass, constant_pool: &mut ConstantPool) -> CompiledMethod {
    let mut local_var_pool = LocalVarPool(vec![]);
    let mut stack = StackSize::new();
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
    stack.inc(1); // aload_0
    stack.dec(1); // invokespecial
    for field in class.fields.iter() {
        if let Some(x) = &field.val {
            code.push(Instruction::aload_0);
            stack.inc(1);
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
            )));
            stack.dec(2);
        }
    }
    code.push(Instruction::r#return);

    let stack_map_table = StackMapTable::new(&code, &[], constant_pool);
    CompiledMethod {
        name: "<init>".to_string(),
        return_type: Type::Void,
        params: vec![],
        max_stack: stack.max,
        max_locals: 1 + local_var_pool.0.len() as u16,
        code,
        stack_map_table,
    }
}

#[derive(Debug)]
pub struct ConstantPool(Vec<Constant>, String);
impl ConstantPool {
    pub fn new(name: String) -> Self {
        // This is the same boilerplate constantpool for all files
        // so we can just hardcode it here.
        // This would only ever change, if we allowed the user
        // to create their own constructors, which we don't
        Self(
            vec![
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
                Constant::Class(name.clone()),
                Constant::Utf8(name.clone()),
                Constant::Utf8("Code".to_string()),
            ],
            name,
        )
    }
    // For some unknown reason, this is 1-indexed and we have to add 1 to the count
    pub fn count(&self) -> u16 {
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
            Constant::String(str) => {
                self.add(Constant::Utf8(str));
            }
            Constant::Integer(int) => {}
            Constant::Utf8(name) => {}
        };
        self.0.push(constant);
        self.0.len() as u16
    }
    pub fn index_of(&self, constant: &Constant) -> Option<u16> {
        self.0
            .iter()
            .position(|c| *c == *constant)
            .map(|x| x as u16 + 1) // +1 because the constant pool is 1-indexed
    }
    pub fn index_of_this_class(&self) -> u16 {
        self.index_of(&Constant::Class(self.1.clone())).unwrap()
    }
    /// Returns the constant at the given index. Note that this is 1-indexed since the constant
    /// pool of the JVM is 1-indexed
    pub fn get(&self, index: u16) -> Option<&Constant> {
        self.0.get(index as usize - 1)
    }
    /// See this table: https://docs.oracle.com/javase/specs/jvms/se15/html/jvms-4.html#jvms-4.4
    pub fn as_bytes(&mut self) -> Vec<u8> {
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
                            .index_of(&Constant::Utf8(val.clone()))
                            .unwrap()
                            .to_be_bytes(),
                    );
                }
                Constant::Integer(int) => {
                    result.push(3);
                    result.extend_from_slice(&int.to_be_bytes());
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
        // +1 because the 0th local variable is `this`, which isn't captured in this structure
        1 + self
            .0
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
    pub(crate) stack_map_table: StackMapTable,
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
        // attr = attribute after attribute_length
        let mut attr = vec![];
        attr.extend_from_slice(&self.max_stack.to_be_bytes());
        attr.extend_from_slice(&self.max_locals.to_be_bytes());
        let mut code_bytes = vec![];
        self.code
            .iter()
            .for_each(|i| code_bytes.append(&mut i.as_bytes()));
        attr.extend_from_slice(&(code_bytes.len() as u32).to_be_bytes());
        attr.append(&mut code_bytes);
        attr.extend_from_slice(&[0, 0]); // Exception table length

        // Inner Attributes (only StackMapTable)
        // First count, then the each attribute
        if self.stack_map_table.is_implicit() {
            attr.extend_from_slice(&[0, 0]);
        } else {
            attr.extend_from_slice(&[0, 1]);
            attr.append(&mut self.stack_map_table.as_bytes(constant_pool))
        }
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
    String(String),
    Utf8(String),
    Integer(i32), // Used only for when the integer is too big to fit into a i16
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

fn get_instruction_length(istr: &Instruction) -> u16 {
    match istr {
        i => i.as_bytes().len() as u16,
    }
}

fn get_instructions_length(instructions: &[Instruction]) -> u16 {
    instructions.iter().map(get_instruction_length).sum()
}

/// The instructions for the JVM
/// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-6.html#jvms-6.5.areturn
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum Instruction {
    invokespecial(u16), //Calling a method from the super class (probably only used in constructor)
    aload_0,
    aload(u8),   //Load reference from local variable
    iload(u8),   //Load int from local variable
    ireturn,     //return int, char, boolean
    r#return,    //return void
    areturn,     //return object(string, integer, null)
    bipush(i8),  //Push signed byte onto stack
    sipush(i16), //Push signed short onto stack
    istore(u8),  //Store int into local variable
    astore(u8),  //Store reference into local variable
    aconst_null, //Push null onto stack
    ldc(u8), //Push item from constant pool onto stack - For some reason only one byte for index into constant pool :shrug:
    ineg,    //Negate int
    // @Note: All absolute jumps store first the relative offset in bytes and then in instructions
    ifeq(i16, i16), //Branch if int is 0
    iflt(i16, i16), //Branch if int is < 0
    ifge(i16, i16), //Branch if int is >= 0
    ifne(i16, i16), //Branch if int is not 0
    goto(i16, i16), //Jump to instruction
    iadd,           //Add int
    isub,           //Subtract int
    imul,           //Multiply int
    idiv,           //Divide int
    irem,           //Remainder int
    putfield(u16), //Sets a value for the field at the given index. The stack must have the reference to the object to which the field belongs and on top of that the value to set the field to
    getfield(u16), // Get field from object via an index into the constant pool
    new(u16),      //Create new object
    dup,           //Duplicate the top value on the stack
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
            Instruction::ireturn => vec![172],
            Instruction::r#return => vec![177],
            Instruction::areturn => vec![176],
            Instruction::bipush(byte) => vec![16, *byte as u8],
            Instruction::sipush(short) => {
                vec![17, high_byte(*short as u16), low_byte(*short as u16)]
            }
            Instruction::istore(idx) => vec![54, *idx],
            Instruction::astore(idx) => vec![58, *idx],
            Instruction::aconst_null => vec![1],
            Instruction::ldc(idx) => vec![18, *idx],
            Instruction::ineg => vec![116],
            Instruction::ifeq(jmp_in_bytes, _jmp_in_inst) => {
                vec![153, shigh_byte(*jmp_in_bytes), slow_byte(*jmp_in_bytes)]
            }
            Instruction::ifne(jmp_in_bytes, _jmp_in_inst) => {
                vec![154, shigh_byte(*jmp_in_bytes), slow_byte(*jmp_in_bytes)]
            }
            Instruction::ifge(jmp_in_bytes, _jmp_in_inst) => {
                vec![156, shigh_byte(*jmp_in_bytes), slow_byte(*jmp_in_bytes)]
            }
            Instruction::iflt(jmp_in_bytes, _jmp_in_inst) => {
                vec![155, shigh_byte(*jmp_in_bytes), slow_byte(*jmp_in_bytes)]
            }
            Instruction::goto(jmp_in_bytes, _jmp_in_inst) => {
                vec![167, shigh_byte(*jmp_in_bytes), slow_byte(*jmp_in_bytes)]
            }
            Instruction::iadd => vec![96],
            Instruction::isub => vec![100],
            Instruction::imul => vec![104],
            Instruction::idiv => vec![108],
            Instruction::irem => vec![112],
            Instruction::putfield(idx) => vec![181, high_byte(*idx), low_byte(*idx)],
            Instruction::getfield(idx) => vec![180, high_byte(*idx), low_byte(*idx)],
            Instruction::new(idx) => vec![187, high_byte(*idx), low_byte(*idx)],
            Instruction::dup => vec![89],
            e => panic!("Instruction {:?} not implemented or unexpected", e),
        }
    }
}

pub fn generate_dir(ast: &Prg) -> DIR {
    let mut dir = DIR {
        constant_pool: ConstantPool::new(ast.get(0).unwrap().name.clone()),
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

// @Cleanup this function is never used
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
    let mut code = generate_code_stmt(
        method.body.clone(),
        &mut stack,
        constant_pool,
        &mut local_var_pool,
        class_name,
        true,
    );

    if code.last().unwrap_or(&Instruction::bipush(0)) != &Instruction::r#return
        && method.ret_type == Type::Void
    {
        code.push(Instruction::r#return);
    }

    let stack_map_table = StackMapTable::new(&code, &method.params, constant_pool);
    CompiledMethod {
        name: method.name.clone(),
        return_type: method.ret_type.clone(),
        params: method.params.clone(),
        max_stack: stack.max,
        max_locals: 1 + local_var_pool.0.len() as u16,
        code,
        stack_map_table,
    }
}

fn generate_code_stmt(
    stmt: Stmt,
    stack: &mut StackSize,
    constant_pool: &mut ConstantPool,
    local_var_pool: &mut LocalVarPool,
    class_name: &str,
    is_last_instr: bool,
) -> Vec<Instruction> {
    let mut result = vec![];
    match stmt {
        Stmt::TypedStmt(stmt, stmt_type) => {
            // Generate bytecode for typed stmt
            let stmt = stmt.deref().clone();
            match stmt {
                Stmt::Block(stmts) => {
                    for i in 0..stmts.len() {
                        result.append(&mut generate_code_stmt(
                            stmts.get(i).unwrap().clone(),
                            stack,
                            constant_pool,
                            local_var_pool,
                            class_name,
                            is_last_instr && i + 1 == stmts.len(),
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
                    let mut cond =
                        generate_code_expr(expr, stack, constant_pool, local_var_pool, class_name);
                    let cond_len = get_instructions_length(&cond) as i16;
                    result.append(&mut cond);
                    // Checking the condition removes one element from stack
                    stack.dec(1);
                    // Generate bytecode for our body
                    let mut body = generate_code_stmt(
                        *stmt,
                        stack,
                        constant_pool,
                        local_var_pool,
                        class_name,
                        is_last_instr,
                    );
                    let body_len = get_instructions_length(&body) as i16;
                    result.push(Instruction::ifeq(3 + body_len, body.len() as i16 + 1));
                    result.append(&mut body);
                    result.push(Instruction::goto(
                        -3 - body_len - cond_len,
                        -(body.len() as i16) - 1,
                    ));
                }
                Stmt::LocalVarDecl(types, name) => {
                    local_var_pool.add(name.clone());
                    stack.inc(1);
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
                    stack.dec(1);
                    let mut if_body = generate_code_stmt(
                        *stmt1,
                        stack,
                        constant_pool,
                        local_var_pool,
                        class_name,
                        is_last_instr,
                    );
                    let mut else_body = if stmt2.is_none() {
                        vec![]
                    } else {
                        generate_code_stmt(
                            *stmt2.clone().unwrap(),
                            stack,
                            constant_pool,
                            local_var_pool,
                            class_name,
                            is_last_instr,
                        )
                    };

                    // We only want to put a goto to after the else-block if there is another instruction after this one
                    if stmt2.is_some() && !is_last_instr {
                        // If there is an else block we need to jump over it at the end of
                        // the if block since the stack could be changed
                        if_body.push(Instruction::goto(
                            3 + get_instructions_length(&else_body) as i16,
                            else_body.len() as i16,
                        ));
                    }
                    // If the expression is false, jump over the if-body
                    result.push(Instruction::ifeq(
                        3 + get_instructions_length(&if_body) as i16,
                        1 + if_body.len() as i16,
                    ));
                    result.append(&mut if_body);
                    // If there is an else block, append it
                    if stmt2.is_some() {
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
                                stack.dec(1);
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
                                stack.inc(1);
                                result.append(&mut expr_code);
                                result.push(Instruction::putfield(idx));
                                stack.dec(2);
                                result.push(Instruction::aload_0);
                                result.push(Instruction::getfield(idx));
                                stack.inc(1);
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
                                stack.dec(2);
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
                    stack.inc(1);
                    result.push(Instruction::invokespecial(method_index));
                    stack.dec(1);
                }
                StmtExpr::MethodCall(_expr, name, args) => {
                    // Generate bytecode for method call
                    // Principally this should work this way:
                    // 1. Write Function Name into Constant Pool generating the necessary Constants
                    // 2. Push all arguments onto the stack
                    // 3. Call invokespecial on the given back function index
                    result.push(Instruction::aload_0);
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
                        format!("({}){}", argument_types, return_type.to_ir_string())
                    }
                    let method_index = constant_pool.add(Constant::MethodRef(MethodRef {
                        class: class_name.to_string(),
                        method: NameAndType {
                            name: name.clone(),
                            r#type: generate_name_and_type(expr_type, args),
                        },
                    }));
                    result.push(Instruction::invokespecial(method_index));
                    stack.inc(1);
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
                    if i < i8::MAX as i32 && i > i8::MIN as i32 {
                        result.push(Instruction::bipush(i as i8));
                    } else if i < i16::MAX as i32 && i > i16::MIN as i32 {
                        result.push(Instruction::sipush(i as i16));
                    } else {
                        result.push(Instruction::ldc(
                            constant_pool.add(Constant::Integer(i)) as u8
                        ));
                    }
                    stack.inc(1);
                }
                Expr::Bool(b) => {
                    result.push(Instruction::bipush(b as i8));
                    stack.inc(1);
                }
                Expr::Char(c) => {
                    result.push(Instruction::bipush(c as i8));
                    stack.inc(1);
                }
                Expr::String(s) => {
                    let index = constant_pool.add(Constant::String(s.to_string()));
                    result.push(Instruction::ldc(index as u8));
                    stack.inc(1);
                }
                Expr::Jnull => {
                    result.push(Instruction::aconst_null);
                    stack.inc(1);
                }
                Expr::This => {
                    result.push(Instruction::aload(0));
                    stack.inc(1);
                }
                Expr::InstVar(exprs, name) => {
                    match exprs.deref() {
                        Expr::TypedExpr(expr, r#type) => match expr.deref() {
                            Expr::This => {
                                result.push(Instruction::aload(0));
                                stack.inc(1);
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
                                stack.inc(3);
                            }
                            Expr::FieldVar(name) => {
                                let field_index = constant_pool.add(Constant::FieldRef(FieldRef {
                                    class: class_name.to_string(),
                                    field: NameAndType {
                                        name: name.clone(),
                                        r#type: r#type.to_ir_string(),
                                    },
                                }));
                                result.push(Instruction::aload_0);
                                result.push(Instruction::getfield(field_index));
                                stack.inc(3);
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
                    stack.inc(2);
                }
                Expr::Binary(op, left, right) => {
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
                            let mut left_code = generate_code_expr(
                                *left,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            );
                            let mut right_code = generate_code_expr(
                                *right,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            );

                            //   ...left
                            //   ifeq (FALSE)
                            //   ...right
                            //   ifeq (FALSE)
                            //   bipush 1
                            //   goto 2
                            // FALSE:
                            //   bipush 0

                            // If left operand is false (== 0), return false immediately
                            let right_len = get_instructions_length(&right_code) as i16;
                            result.append(&mut left_code);
                            result.push(Instruction::ifeq(
                                2 + right_len + 3 + 2 + 3 + 1,
                                4 + right_code.len() as i16,
                            ));
                            // If right operand is false (== 0), return false
                            result.append(&mut right_code);
                            result.push(Instruction::ifeq(2 + 2 + 3 + 1, 3));
                            result.push(Instruction::bipush(1));
                            result.push(Instruction::goto(2 + 2 + 1, 2));
                            result.push(Instruction::bipush(0));
                        }
                        BinaryOp::Or => {
                            let mut left_code = generate_code_expr(
                                *left,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            );
                            let mut right_code = generate_code_expr(
                                *right,
                                stack,
                                constant_pool,
                                local_var_pool,
                                class_name,
                            );
                            result.append(&mut left_code);
                            // If left operand is true (!= 0), return true immediately
                            result.push(Instruction::ifne(
                                11 + get_instructions_length(&right_code) as i16,
                                4 + right_code.len() as i16,
                            ));
                            // If right operand is true (!= 0) return true
                            result.append(&mut right_code);
                            result.push(Instruction::ifne(8, 3));
                            result.push(Instruction::bipush(0));
                            result.push(Instruction::goto(5, 2));
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
                            // a <= b
                            // a - b <= 0
                            // a - b - 1 < 0
                            result.push(Instruction::isub);
                            result.push(Instruction::iload(1));
                            result.push(Instruction::isub);
                            result.push(Instruction::iflt(8, 3));
                            result.push(Instruction::bipush(0));
                            result.push(Instruction::goto(5, 2));
                            result.push(Instruction::bipush(1));
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
                            result.push(Instruction::isub);
                            result.push(Instruction::ifge(8, 3));
                            result.push(Instruction::bipush(0));
                            result.push(Instruction::goto(5, 2));
                            result.push(Instruction::bipush(1));
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
                            result.push(Instruction::isub);
                            result.push(Instruction::iflt(8, 3));
                            result.push(Instruction::bipush(0));
                            result.push(Instruction::goto(2 + 2 + 1, 2));
                            result.push(Instruction::bipush(1));
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
                            // a >= b
                            // a - b >= 0
                            // a - b + 1 > 0
                            result.push(Instruction::isub);
                            result.push(Instruction::bipush(1));
                            result.push(Instruction::iadd);
                            result.push(Instruction::ifge(8, 3));
                            result.push(Instruction::bipush(0));
                            result.push(Instruction::goto(5, 2));
                            result.push(Instruction::bipush(1));
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
                            result.push(Instruction::isub);
                            result.push(Instruction::ifeq(2 + 2 + 3 + 1, 3));
                            result.push(Instruction::bipush(0));
                            result.push(Instruction::goto(2 + 2 + 1, 2));
                            result.push(Instruction::bipush(1));
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
                            result.push(Instruction::ifeq(8, 3));
                            result.push(Instruction::bipush(1));
                            result.push(Instruction::goto(5, 2));
                            result.push(Instruction::bipush(0))
                        }
                    }
                    stack.dec(1);
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
                            result.push(Instruction::ifne(8, 3));
                            result.push(Instruction::bipush(1));
                            result.push(Instruction::goto(5, 2));
                            result.push(Instruction::bipush(0));
                        }
                        UnaryOp::Neg => {
                            result.push(Instruction::ineg);
                        }
                        UnaryOp::Pos => {}
                    }
                }
                Expr::LocalVar(name) => {
                    stack.inc(1);
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
                            r#type: r#type.to_ir_string(),
                        },
                    }));
                    // We only do getfield here because we don't know what operation we're doing
                    // with the field
                    result.push(Instruction::aload_0);
                    result.push(Instruction::getfield(index));
                    stack.inc(2);
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
