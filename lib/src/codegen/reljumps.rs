use crate::codegen::Instruction;
use std::cmp::{max, min};

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
    for (j, istr) in instructions.iter().enumerate() {
        let current_offset = get_instructions_length(&instructions[..j]);
        match istr {
            Instruction::relgoto(target) => {
                let absolute_addr =
                    calculate_absolute_addr(*target, current_offset, j, &instructions);
                result.push(Instruction::goto(absolute_addr));
            }
            Instruction::reljumpifeq(target) => {
                let absolute_addr =
                    calculate_absolute_addr(*target, current_offset, j, &instructions);
                result.push(Instruction::ifeq(absolute_addr));
            }
            Instruction::reljumpifge(target) => {
                let absolute_addr =
                    calculate_absolute_addr(*target, current_offset, j, &instructions);
                result.push(Instruction::ifge(absolute_addr));
            }
            Instruction::reljumpiflt(target) => {
                let absolute_addr =
                    calculate_absolute_addr(*target, current_offset, j, &instructions);
                result.push(Instruction::iflt(absolute_addr));
            }
            Instruction::reljumpifne(target) => {
                let absolute_addr =
                    calculate_absolute_addr(*target, current_offset, j, &instructions);
                result.push(Instruction::ifne(absolute_addr));
            }
            _ => result.push(*istr),
        }
    }
    result
}

fn calculate_absolute_addr(
    target: i16,
    current_offset: u16,
    j: usize,
    instructions: &[Instruction],
) -> u16 {
    let modifier: i32 = if target < 0 { -1 } else { 1 };
    let min = min(j, j.saturating_add_signed(target as isize));
    let max = max(j, j.saturating_add_signed(target as isize));
    (get_instructions_length(&instructions[min..max]) as i32 * modifier + current_offset as i32)
        as u16
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
        expected.push(Instruction::ifne(if_block_size + expected_size));
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
            Instruction::ifne(0),
            Instruction::bipush(1),
            Instruction::ireturn,
            Instruction::bipush(0),
            Instruction::ireturn,
        ];
        assert_eq!(convert_to_absolute_jumps(instructions), expected);
    }
}
