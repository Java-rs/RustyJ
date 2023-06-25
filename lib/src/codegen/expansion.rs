/// Expands the relative jumps in a given bytecode into absolute jumps. Necessary before converting
/// to bytecode
fn expand_function(instructions: Vec<Instruction>) -> Vec<Instruction> {
    // We need to expand the instructions between the relative jump origin and target in order to calculate the actual offset,
    // then we replace the relative jump with an actual respective jump instruction to the correct address
    // TODO: Is this a good idea to do a full pass over the instructions to find the jumps? Maybe we can do it in the same pass as the
    // conversion to bytecode or when generating the instructions
}
