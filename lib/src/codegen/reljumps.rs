use crate::codegen::Instruction;

fn get_instruction_length(istr: &Instruction) -> u16 {
    match istr {
        Instruction::reljumpifeq(_) => 3,
        Instruction::reljumpifge(_) => 3,
        Instruction::relgoto(_) => 3,
        Instruction::reljumpiflt(_) => 3,
        Instruction::reljumpifne(_) => 3,
        i => i.as_bytes().len() as u16,
    }
}

fn get_instructions_length(instructions: &[Instruction]) -> u16 {
    instructions.iter().map(get_instruction_length).sum()
}
/// Converts relative jumps to absolute jumps. Has to be called before bytecode generation.
/// Some context: This function only works because we know the size of the final instructions.
/// We measure the size of the instructions to jump over and then replace the relative jump with
/// size_of_instructions_to_jump_over (*-1 if negative)+ current_position
pub(crate) fn convert_to_absolute_jumps(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let mut result = vec![];
    let mut i = 0;
    for (j, istr) in instructions.iter().enumerate() {
        match istr {
            Instruction::reljumpifeq(target) => {
                let modifier: i32 = if *target < 0 { -1 } else { 1 };
                result.push(Instruction::ifeq(
                    (get_instructions_length(
                        &instructions[j..j.saturating_add_signed(*target as isize)],
                    ) as i32
                        * modifier
                        + j as i32) as u16,
                ))
            }
            Instruction::reljumpifge(target) => {
                let modifier: i32 = if *target < 0 { -1 } else { 1 };
                result.push(Instruction::ifge(
                    (get_instructions_length(
                        &instructions[j..j.saturating_add_signed(*target as isize)],
                    ) as i32
                        * modifier
                        + j as i32) as u16,
                ))
            }
            Instruction::relgoto(target) => {
                let modifier: i32 = if *target < 0 { -1 } else { 1 };
                result.push(Instruction::goto(
                    (get_instructions_length(
                        &instructions[j..j.saturating_add_signed(*target as isize)],
                    ) as i32
                        * modifier
                        + j as i32) as u16,
                ))
            }
            Instruction::reljumpiflt(target) => {
                let modifier: i32 = if *target < 0 { -1 } else { 1 };
                result.push(Instruction::iflt(
                    (get_instructions_length(
                        &instructions[j..j.saturating_add_signed(*target as isize)],
                    ) as i32
                        * modifier
                        + j as i32) as u16,
                ))
            }
            Instruction::reljumpifne(target) => {
                let modifier: i32 = if *target < 0 { -1 } else { 1 };
                result.push(Instruction::ifne(
                    (get_instructions_length(
                        &instructions[j..j.saturating_add_signed(*target as isize)],
                    ) as i32
                        * modifier
                        + j as i32) as u16,
                ))
            }
            _ => result.push(*istr),
        }
    }
    result
}
