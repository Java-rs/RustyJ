use crate::typechecker::*;
use crate::types::*;
/// The DIR(Duck Intermediate Representation) is our IR for generating Java Bytecode
/// from our TAST
pub(crate) struct DIR {
    pub(crate) constant_pool: Vec<Constant>,
    pub(crate) classes: Vec<IRClass>,
}
pub(crate) struct IRClass {
    pub(crate) name: String,
    pub(crate) super_name: String,
    pub(crate) fields: Vec<FieldDecl>,
    pub(crate) methods: Vec<CompiledMethod>,
}

pub(crate) struct CompiledMethod {
    pub(crate) name: String,
    pub(crate) max_stack: u16,
    pub(crate) code: Vec<Instruction>,
}
pub(crate) struct Constant {
    pub(crate) tag: u8,
    pub(crate) data: Vec<u8>,
}

pub fn generate_dir(ast: &Prg) -> DIR {
    let mut dir = DIR {
        constant_pool: vec![],
        classes: vec![],
    };
    for class in ast {
        dir.classes.push(generate_class(class, &mut dir));
    }
    dir
}

fn generate_class(class: &Class, dir: &mut DIR) -> IRClass {
    let mut ir_class = IRClass {
        name: class.name.clone(),
        super_name: class.superclass.clone(),
        fields: vec![],
        methods: vec![],
    };
    for field in &class.fields {
        ir_class.fields.push(field.clone());
    }
    for method in &class.methods {
        ir_class.methods.push(generate_method(method, dir));
    }
    ir_class
}
// TODO: Parallelize this, since methods are not dependent on each other(hopefully)
fn generate_method(method: &MethodDecl, dir: &mut DIR) -> CompiledMethod {
    let mut compiled_method = CompiledMethod {
        name: method.name.clone(),
        max_stack: 0,
        code: vec![],
    };
    for stmt in &method.body {
        compiled_method.code.append(&mut generate_stmt(stmt, dir));
    }
    compiled_method
}

fn generate_stmt(stmt: Stmt, dir: &mut DIR) -> Vec<Instruction> {
    todo!()
}
