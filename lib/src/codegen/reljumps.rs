use crate::codegen::Instruction;

fn get_instruction_length(istr: &Instruction) -> u16 {
    match istr {
        Instruction::reljumpifeq(_)
        | Instruction::reljumpifge(_)
        | Instruction::relgoto(_)
        | Instruction::reljumpiflt(_)
        | Instruction::reljumpifne(_) => 3,
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
    for (j, istr) in instructions.iter().enumerate() {
        match istr {
            Instruction::reljumpifeq(target) => {
                let modifier: i32 = if *target < 0 { -1 } else { 1 };
                result.push(Instruction::ifeq(
                    (get_instructions_length(
                        &instructions[j..j.saturating_add_signed(*target as isize)],
                    ) as i32
                        * modifier
                        + j as i32
                        - 1) as u16,
                    *target,
                ))
            }
            Instruction::reljumpifge(target) => {
                let modifier: i32 = if *target < 0 { -1 } else { 1 };
                result.push(Instruction::ifge(
                    (get_instructions_length(
                        &instructions[j..j.saturating_add_signed(*target as isize)],
                    ) as i32
                        * modifier
                        + j as i32
                        - 1) as u16,
                    *target,
                ))
            }
            Instruction::relgoto(target) => {
                let modifier: i32 = if *target < 0 { -1 } else { 1 };
                result.push(Instruction::goto(
                    (get_instructions_length(
                        &instructions[j..j.saturating_add_signed(*target as isize)],
                    ) as i32
                        * modifier
                        + j as i32
                        - 1) as u16,
                    *target,
                ))
            }
            Instruction::reljumpiflt(target) => {
                let modifier: i32 = if *target < 0 { -1 } else { 1 };
                result.push(Instruction::iflt(
                    (get_instructions_length(
                        &instructions[j..j.saturating_add_signed(*target as isize)],
                    ) as i32
                        * modifier
                        + j as i32
                        - 1) as u16,
                    *target,
                ))
            }
            Instruction::reljumpifne(target) => {
                let modifier: i32 = if *target < 0 { -1 } else { 1 };
                result.push(Instruction::ifne(
                    (get_instructions_length(
                        &instructions[j..j.saturating_add_signed(*target as isize)],
                    ) as i32
                        * modifier
                        + j as i32
                        - 1) as u16,
                    *target,
                ))
            }
            _ => result.push(*istr),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_if_expansion() {
        let instructions = vec![
            Instruction::iload(1),
            Instruction::iload(2),
            Instruction::isub,
            Instruction::reljumpifne(3),
            Instruction::bipush(1),
            Instruction::ireturn,
            Instruction::bipush(0),
            Instruction::ireturn,
        ];
        let mut expected = vec![
            Instruction::iload(1),
            Instruction::iload(2),
            Instruction::isub,
        ];
        let if_block = vec![Instruction::bipush(1), Instruction::ireturn];
        let if_block_size = get_instructions_length(&if_block);
        let expected_size = get_instructions_length(&expected);
        expected.push(Instruction::ifne(if_block_size + expected_size, 3));
        expected.extend(if_block);
        expected.extend_from_slice(&[Instruction::bipush(0), Instruction::ireturn]);
        assert_eq!(convert_to_absolute_jumps(instructions), expected);
    }
    #[test]
    fn test_negative_expansion() {
        let instructions = vec![
            Instruction::iload(1),
            Instruction::iload(2),
            Instruction::isub,
            Instruction::reljumpifne(-3),
            Instruction::bipush(1),
            Instruction::ireturn,
            Instruction::bipush(0),
            Instruction::ireturn,
        ];
        let expected = vec![
            Instruction::iload(1),
            Instruction::iload(2),
            Instruction::isub,
            Instruction::ifne(0, -3),
            Instruction::bipush(1),
            Instruction::ireturn,
            Instruction::bipush(0),
            Instruction::ireturn,
        ];
        assert_eq!(convert_to_absolute_jumps(instructions), expected);
    }
}
