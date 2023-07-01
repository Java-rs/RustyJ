#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(non_snake_case)]

use super::*;
use crate::types::*;
use std::fmt::Debug;

#[derive(Debug)]
pub struct StackSize {
    pub current: u16,
    pub max: u16,
}

impl StackSize {
    pub fn new() -> Self {
        StackSize { current: 0, max: 0 }
    }

    pub fn inc(&mut self, step: u16) {
        self.current += step;
        if self.current > self.max {
            self.max = self.current;
        }
    }

    pub fn dec(&mut self, step: u16) {
        self.current -= step;
    }

    pub fn set(&mut self, val: u16) {
        self.current = val;
        if self.current > self.max {
            self.max = self.current;
        }
    }
}

// Sorts the vector and removes any duplicate elements
pub fn sort_unique<T, F1, F2>(a: &mut Vec<T>, mut eq: F1, mut le: F2)
where
    F1: FnMut(&T, &T) -> bool,
    F2: FnMut(&T, &T) -> bool,
{
    let mut i = 0;
    while i < a.len() {
        let mut min = i;
        for j in i + 1..a.len() {
            if le(&a[j], &a[min]) {
                min = j;
            }
        }

        if i != 0 && eq(&a[i - 1], &a[min]) {
            a.swap_remove(min);
        } else {
            if min != i {
                a.swap(i, min);
            }
            i += 1;
        }
    }
}

// Good explanation of what a StackMapTable is and why it exists: https://stackoverflow.com/a/25110513
// Documentation for StackMapTable: https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.7.4
// Basic Idea: It exists to simplify the typechecking of the bytecode performed by the JVM
// For each jump-location, there exists one frame in the stack table, which is described in relation to the previous frame
#[derive(Debug)]
pub(crate) struct StackMapTable {
    frames: Vec<StackMapFrame>,
}

impl StackMapTable {
    fn create_stacks_for_branch(
        code: &[Instruction],
        mut instruction_idx: usize,
        mut bytes_idx: usize,
        current_stack: &mut VerificationStack,
        stacks: &mut Vec<VerificationStack>,
        taken_branches_idxs: &mut Vec<usize>, // indexes are for instructon vector
        constant_pool: &mut ConstantPool,
    ) {
        while instruction_idx < code.len() {
            // FIXME: When are locals added and how can we detect that from the instructions alone?
            match code.get(instruction_idx).unwrap() {
                Instruction::invokespecial(idx) => {
                    bytes_idx += 3;
                    if let Some(Constant::MethodRef(m)) = constant_pool.get(*idx) {
                        // FIXME: Somehow figure out how many parameters are popped when calling the given function
                        let pop_amount = 1;
                        let l = current_stack.operands.len();
                        current_stack.operands.splice(l - pop_amount..l, []);
                    } else {
                        unreachable!();
                    }
                }
                Instruction::ldc(idx) => {
                    bytes_idx += 2;
                    match constant_pool.get(*idx as u16).unwrap() {
                        Constant::String(_) => {
                            current_stack.operands.push(VerificationType::OBJECT(
                                constant_pool.add(Constant::Utf8("Ljava/lang/String".to_string())),
                            ))
                        }
                        Constant::Integer(_) => {
                            current_stack.operands.push(VerificationType::INTEGER)
                        }
                        Constant::FieldRef(f) => {
                            current_stack.operands.push(match f.field.r#type.as_str() {
                                "Z" | "C" | "I" => VerificationType::INTEGER,
                                "Ljava/lang/String" => VerificationType::OBJECT(
                                    constant_pool
                                        .index_of(&Constant::Utf8("Ljava/lang/String".to_string()))
                                        .unwrap(),
                                ),
                                _ => VerificationType::OBJECT(constant_pool.index_of_this_class()),
                            })
                        }
                        _ => unreachable!(),
                    }
                }
                Instruction::aconst_null => {
                    bytes_idx += 1;
                    current_stack.operands.push(VerificationType::NULL)
                }
                Instruction::new(_) => todo!(), // requires special treatment, I think, see documentation VerificationType::UNINITIALIZED
                Instruction::aload_0 => {
                    bytes_idx += 1;
                    current_stack.operands.push(VerificationType::OBJECT(
                        constant_pool.index_of_this_class(),
                    ))
                }
                Instruction::aload(idx) => {
                    bytes_idx += 2;
                    current_stack
                        .operands
                        .push(VerificationType::OBJECT(*idx as u16))
                }
                Instruction::iload(_) | Instruction::bipush(_) => {
                    bytes_idx += 2;
                    current_stack.operands.push(VerificationType::INTEGER)
                }
                Instruction::sipush(_) => {
                    bytes_idx += 3;
                    current_stack
                        .operands
                        .extend_from_slice(&[VerificationType::INTEGER, VerificationType::INTEGER])
                }
                Instruction::ireturn | Instruction::r#return | Instruction::areturn => {
                    bytes_idx += 1;
                    current_stack.operands.clear()
                }
                Instruction::putfield(_) => {
                    bytes_idx += 3;
                    current_stack.operands.pop();
                    current_stack.operands.pop();
                }
                Instruction::ineg => {
                    bytes_idx += 1;
                    // No changes in stack
                }

                // Locals
                // this x
                Instruction::istore(x) => {
                    bytes_idx += 2;
                    current_stack.operands.pop();
                    if *x as usize >= current_stack.locals.len() {
                        current_stack.locals.push(VerificationType::INTEGER)
                    }
                }
                Instruction::astore(x) => {
                    let a = current_stack.operands.pop().unwrap();
                    if *x as usize >= current_stack.locals.len() {
                        current_stack.locals.push(a);
                    }
                }
                Instruction::iadd
                | Instruction::isub
                | Instruction::imul
                | Instruction::idiv
                | Instruction::irem => {
                    bytes_idx += 1;
                    current_stack.operands.pop();
                }
                Instruction::getfield(idx) => {
                    // Use idx to figure out what type the field has
                    // pops once and pushes type of field then
                    bytes_idx += 3;
                    current_stack.operands.pop();
                    if let Constant::FieldRef(f) = constant_pool.get(*idx).unwrap() {
                        match f.field.r#type.as_str() {
                            "Z" | "C" | "I" => {
                                current_stack.operands.push(VerificationType::INTEGER)
                            }
                            "Ljava/lang/String" => {
                                current_stack.operands.push(VerificationType::OBJECT(
                                    constant_pool
                                        .index_of(&Constant::Utf8("Ljava/lang/String".to_string()))
                                        .unwrap(),
                                ))
                            }
                            _ => current_stack.operands.push(VerificationType::OBJECT(
                                constant_pool.index_of_this_class(),
                            )),
                        }
                    } else {
                        unreachable!();
                    }
                }
                Instruction::dup => {
                    bytes_idx += 1;
                    let last = current_stack.operands.last().unwrap();
                    current_stack.operands.push(last.clone());
                }
                ////// JUMPS
                ////// Here it gets interesting
                // @Note: byte_offset is offset in bytes-vector, instruction_offset is relative offset instructions-vector
                Instruction::ifeq(byte_offset, instruction_offset)
                | Instruction::iflt(byte_offset, instruction_offset)
                | Instruction::ifge(byte_offset, instruction_offset)
                | Instruction::ifne(byte_offset, instruction_offset) => {
                    assert!(
                        *instruction_offset != 0,
                        "Instruction {:?} has instruction_offset 0",
                        code[instruction_idx]
                    );
                    if taken_branches_idxs.iter().any(|x| *x == instruction_idx) {
                        return;
                    }
                    taken_branches_idxs.push(instruction_idx);
                    // assert!(*instruction_offset != 0);
                    current_stack.operands.pop();
                    let mut new_stack = current_stack.clone();
                    new_stack.location = (bytes_idx as i16 + *byte_offset) as u16;
                    stacks.push(new_stack.clone());
                    Self::create_stacks_for_branch(
                        code,
                        (instruction_idx as i16 + *instruction_offset) as usize,
                        new_stack.location as usize,
                        &mut new_stack,
                        stacks,
                        taken_branches_idxs,
                        constant_pool,
                    );
                    bytes_idx += 3;
                }
                Instruction::goto(byte_offset, instruction_offset) => {
                    // -1 because +1 is added at the end of the loop again
                    if taken_branches_idxs.iter().any(|x| *x == instruction_idx) {
                        return;
                    }
                    taken_branches_idxs.push(instruction_idx);
                    instruction_idx = (instruction_idx as i16 + instruction_offset) as usize;
                    current_stack.location = (bytes_idx as i16 + *byte_offset) as u16;
                    stacks.push(current_stack.clone());
                }
            }
            instruction_idx += 1;
        }
    }

    // @Note: Expects the code to already be expanded
    pub(crate) fn new(
        code: &[Instruction],
        params: &[(Type, String)],
        constant_pool: &mut ConstantPool,
    ) -> Self {
        // We calculate the actual frames here
        // We do this via 2 passes
        // First, we create the stacks for all different branches
        // storing the snapshot of the stack for every location that we jump to
        // Second, we create a Frame from each of those stacks
        let mut frames = vec![];
        let mut stacks = vec![];
        let mut initial_locals: Vec<VerificationType> = vec![VerificationType::OBJECT(
            constant_pool.index_of_this_class(),
        )];
        initial_locals.append(
            &mut params
                .iter()
                .map(|(t, _)| match t {
                    Type::Bool | Type::Char | Type::Int => VerificationType::INTEGER,
                    Type::Null => VerificationType::NULL,
                    Type::String => VerificationType::OBJECT(todo!()),
                    Type::Class(name) => VerificationType::OBJECT(
                        constant_pool
                            .index_of(&Constant::Class(name.to_string()))
                            .unwrap(),
                    ),
                    Type::Void => unreachable!(),
                })
                .collect(),
        );

        let mut current_stack = VerificationStack {
            location: 0,
            locals: initial_locals.clone(),
            operands: vec![],
        };
        Self::create_stacks_for_branch(
            code,
            0,
            0,
            &mut current_stack,
            &mut stacks,
            &mut vec![],
            constant_pool,
        );
        sort_unique(
            &mut stacks,
            |a, b| a.location == b.location,
            |a, b| a.location < b.location,
        );

        // First stack/frame are implicit
        let mut last_stack = VerificationStack {
            location: 0,
            locals: initial_locals,
            operands: vec![],
        };
        let mut is_first = true;
        for stack in stacks {
            let offset_delta = stack.location - last_stack.location - if !is_first { 1 } else { 0 };
            is_first = false;
            let frame = if stack == last_stack {
                if offset_delta < 64 {
                    StackMapFrame::SAME(offset_delta as u8)
                } else {
                    StackMapFrame::SAME_EXTENDED(offset_delta)
                }
            } else if stack.locals == last_stack.locals
                && !stack.operands.is_empty()
                && stack.operands[0..stack.operands.len() - 1] == last_stack.operands
            {
                if offset_delta < 64 {
                    StackMapFrame::SAME_LOCALS_1(
                        64 + offset_delta as u8,
                        stack.operands.last().unwrap().clone(),
                    )
                } else {
                    StackMapFrame::SAME_LOCALS_1_EXTENDED(
                        offset_delta,
                        stack.operands.last().unwrap().clone(),
                    )
                }
            } else if stack.operands.is_empty()
                && stack.locals.len() > last_stack.locals.len()
                && stack.locals.len() - last_stack.locals.len() < 4
                && stack.locals[0..last_stack.locals.len()] == last_stack.locals
            {
                StackMapFrame::APPEND(
                    (stack.locals.len() - last_stack.locals.len()) as u8 + 251,
                    offset_delta,
                    stack.locals[last_stack.locals.len()..].to_owned(),
                )
            } else if stack.operands.is_empty()
                && stack.locals.len() < last_stack.locals.len()
                && last_stack.locals.len() - stack.locals.len() < 4
                && stack.locals == last_stack.locals[0..stack.locals.len()]
            {
                StackMapFrame::CHOP(
                    251 - (last_stack.locals.len() - stack.locals.len()) as u8,
                    offset_delta,
                )
            } else {
                StackMapFrame::FULL(
                    offset_delta,
                    stack.locals.len() as u16,
                    stack.locals.to_owned(),
                    stack.operands.len() as u16,
                    stack.operands.to_owned(),
                )
            };
            frames.push(frame);
            last_stack = stack;
        }

        StackMapTable { frames }
    }

    // Every Method has a StackMapTable, hower, the first frame is implicit.
    // If there is only the first frame in the StackMapTable, then the whole table is implicit
    pub(crate) fn is_implicit(&self) -> bool {
        self.frames.is_empty()
    }

    pub(crate) fn as_bytes(&self, constant_pool: &mut ConstantPool) -> Vec<u8> {
        let mut result = vec![];
        // Name index
        result.extend_from_slice(
            &constant_pool
                .add(Constant::Utf8("StackMapTable".to_string()))
                .to_be_bytes(),
        );
        // Attribute
        let mut attr: Vec<u8> = self.frames.iter().map(|x| x.as_bytes()).flatten().collect();
        // Attribute length
        // +2 because the 2 bytes for the entries length should also be included
        result.extend_from_slice(&(2 + attr.len() as u32).to_be_bytes());
        // Entries length
        result.extend_from_slice(&(self.frames.len() as u16).to_be_bytes());
        // Stack Map Frames
        result.append(&mut attr);
        result
    }
}

#[derive(Debug)]
pub(crate) enum StackMapFrame {
    SAME(u8),                                                          // u8 in [0, 63]
    SAME_EXTENDED(u16),                  // tag == 251, u16 is offset_delta
    SAME_LOCALS_1(u8, VerificationType), // u8 in [64, 127]
    SAME_LOCALS_1_EXTENDED(u16, VerificationType), // tag == 247, u16 is offset_delta
    CHOP(u8, u16),                       // u8 in [248, 250], u16 is offset_delta
    APPEND(u8, u16, Vec<VerificationType>), // u8 in [252, 254], u16 is offset_delta, size of VerificationTypeInfos is [u8 - 251]
    FULL(u16, u16, Vec<VerificationType>, u16, Vec<VerificationType>), // tag == 255, offset_delta, number_of_locals, [_; number_of_locals], number_of_stack_items, [_; number_of_stack_items]
}

impl StackMapFrame {
    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        match self {
            StackMapFrame::SAME(offset_delta) => vec![*offset_delta],
            StackMapFrame::SAME_EXTENDED(offset_delta) => {
                vec![251, high_byte(*offset_delta), low_byte(*offset_delta)]
            }
            StackMapFrame::SAME_LOCALS_1(offset_delta, r#type) => {
                let mut v = Vec::with_capacity(8);
                v.push(*offset_delta);
                v.extend_from_slice(&r#type.as_bytes());
                v
            }
            StackMapFrame::SAME_LOCALS_1_EXTENDED(offset_delta, r#type) => {
                let mut v = Vec::with_capacity(8);
                v.push(247);
                v.extend_from_slice(&offset_delta.to_be_bytes());
                v.extend_from_slice(&r#type.as_bytes());
                v
            }
            StackMapFrame::CHOP(chopped_amount, offset_delta) => {
                todo!()
            }
            StackMapFrame::APPEND(appended_amount, offset_delta, types) => {
                let mut v = Vec::with_capacity(16);
                v.push(*appended_amount);
                v.extend_from_slice(&offset_delta.to_be_bytes());
                v.append(&mut types.iter().map(|t| t.as_bytes()).flatten().collect());
                let v = dbg!(v);
                v
            }
            StackMapFrame::FULL(
                offset_delta,
                number_of_locals,
                local_types,
                number_of_operands,
                operand_types,
            ) => {
                let mut v = Vec::with_capacity(32);
                v.push(255);
                v.extend_from_slice(&offset_delta.to_be_bytes());
                v.extend_from_slice(&number_of_locals.to_be_bytes());
                v.append(&mut local_types.iter().map(|t| t.as_bytes()).flatten().collect());
                v.extend_from_slice(&number_of_operands.to_be_bytes());
                v.append(
                    &mut operand_types
                        .iter()
                        .map(|t| t.as_bytes())
                        .flatten()
                        .collect(),
                );
                v
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum VerificationType {
    TOP,
    INTEGER,
    NULL,
    UNINITIALIZED_THIS,
    OBJECT(u16),        // index in constant pool
    UNINITIALIZED(u16), // offset (see docs for specific infos)
}

impl VerificationType {
    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        match self {
            VerificationType::TOP => vec![0],
            VerificationType::INTEGER => vec![1],
            VerificationType::NULL => vec![5],
            VerificationType::UNINITIALIZED_THIS => vec![6],
            VerificationType::OBJECT(cp_idx) => {
                let mut v = Vec::with_capacity(4);
                v.push(7);
                v.extend_from_slice(&cp_idx.to_be_bytes());
                v
            }
            VerificationType::UNINITIALIZED(offset) => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct VerificationStack {
    location: u16,
    locals: Vec<VerificationType>,
    operands: Vec<VerificationType>,
}

impl PartialEq for VerificationStack {
    fn eq(&self, other: &Self) -> bool {
        if self.locals.len() != other.locals.len() || self.operands.len() != other.operands.len() {
            return false;
        }
        for i in 0..self.locals.len() {
            if self.locals.get(i).unwrap() != other.locals.get(i).unwrap() {
                return false;
            }
        }
        for i in 0..self.operands.len() {
            if self.operands.get(i).unwrap() != other.operands.get(i).unwrap() {
                return false;
            }
        }
        true
    }
}
