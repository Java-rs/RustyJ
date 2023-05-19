mod ir;
/// The DIR(Duck Intermediate Representation) is our IR for generating Java Bytecode
/// from our TAST
struct DIR {
    pub(crate) constant_pool: Vec<Constant>,
    pub(crate) classes: Vec<IRClass>,
}
