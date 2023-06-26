use crate::codegen::Instruction;

fn get_instruction_length(istr: &Instruction) -> u16 {
    match istr {
        // TODO: These are just Copilots guesses
        Instruction::reljumpifeq(_) => 3,
        Instruction::reljumpifge(_) => 2,
        Instruction::relgoto(_) => 3,
        Instruction::reljumpiflt(_) => 2,
        i => i.as_bytes().len() as u16,
    }
}

fn get_instructions_length(instructions: &[Instruction]) -> u16 {
    instructions.iter().map(|i| get_instruction_length(i)).sum()
}
/// Converts relative jumps to absolute jumps. Has to be called before bytecode generation.
/// Some context: This function only works because we know the size of the final instructions.
/// We measure the size of the instructions to jump over and then replace the relative jump with
/// size_of_instructions_to_jump_over + current_position
pub(crate) fn convert_to_absolute_jumps(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let mut result = vec![];
    let mut i = 0;
    for (j, istr) in instructions.iter().enumerate() {
        match istr {
            Instruction::reljumpifeq(target) => result.push(Instruction::ifeq(
                get_instructions_length(
                    &instructions[j..j.saturating_add_signed(*target as isize)],
                ) + j as u16,
            )),
            Instruction::reljumpifge(target) => result.push(Instruction::ifge(
                get_instructions_length(
                    &instructions[j..j.saturating_add_signed(*target as isize)],
                ) + j as u16,
            )),
            Instruction::relgoto(target) => result.push(Instruction::goto(
                get_instructions_length(
                    &instructions[j..j.saturating_add_signed(*target as isize)],
                ) + j as u16,
            )),
            Instruction::reljumpiflt(target) => result.push(Instruction::iflt(
                get_instructions_length(
                    &instructions[j..j.saturating_add_signed(*target as isize)],
                ) + j as u16,
            )),
            _ => result.push(istr.clone()),
        }
    }
    result
}
