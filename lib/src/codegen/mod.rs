use crate::types::FieldDecl;

mod ir;
/// The DIR(Duck Intermediate Representation) is our IR for generating Java Bytecode
/// from our TAST
pub(crate) struct DIR {
    pub(crate) constant_pool: Vec<Constant>,
    pub(crate) classes: Vec<IRClass>,
}
pub(crate) struct IRClass {
    pub(crate) name: String,
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
