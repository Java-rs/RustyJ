#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(non_snake_case)]

use super::ir::*;
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
pub fn sort_unique<T>(a: &mut Vec<T>)
where
    T: PartialOrd,
{
    let mut i = 0;
    while i < a.len() {
        let mut min = i;
        for j in i + 1..a.len() {
            if a[j] < a[min] {
                min = j;
            }
        }

        if i != 0 && a[i - 1] == a[min] {
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
    // @Note: Expects the code to already be expanded
    pub(crate) fn new(
        code: &[Instruction],
        params: &[(Type, String)],
        constant_pool: &ConstantPool,
    ) -> Self {
        // We calculate the actual frames here
        // We do this via 2 passes
        // First, we find all locations, that are targets of jumps
        // Second, we create a Frame for each of those locations
        let mut locations = vec![];
        let mut frames = vec![];

        for inst in code {
            match inst {
                Instruction::ifeq(loc)
                | Instruction::iflt(loc)
                | Instruction::ifge(loc)
                | Instruction::ifne(loc)
                | Instruction::goto(loc) => locations.push(*loc),
                _ => {}
            }
        }
        sort_unique(&mut locations);

        if !locations.is_empty() {
            let mut loc_idx = 0;
            let mut last_frame_loc = 0;
            let mut last_stack = VerificationStack {
                locals: params
                    .iter()
                    .map(|(t, _)| match t {
                        Type::Bool => VerificationType::INTEGER,
                        _ => todo!(),
                    })
                    .collect(),
                operands: vec![],
            };
            let mut current_stack = last_stack.clone();

            for i in 0..code.len() {
                current_stack.update(code.get(i).unwrap(), constant_pool);
                if i as u16 == *locations.get(loc_idx).unwrap() {
                    frames.push(current_stack.to_frame(&last_stack, (i - last_frame_loc) as u16));
                    last_stack = current_stack.clone();
                    last_frame_loc = i;

                    loc_idx += 1;
                    if loc_idx == locations.len() {
                        break;
                    }
                }
            }
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
    SAME_LOCALS_1(u8, VerificationType),                               // u8 in [64, 127]
    SAME_LOCALS_1_EXTENDED(u16, VerificationType), // tag == 247, u16 is offset_delta
    CHOP(u8, u16),                                 // u8 in [248, 250], u16 is offset_delta
    SAME_EXTENDED(u16),                            // tag == 251, u16 is offset_delta
    APPEND(u8, u16, Vec<VerificationType>), // u8 in [252, 254], u16 is offset_delta, size of VerificationTypeInfos is [u8 - 251]
    FULL(u16, u16, Vec<VerificationType>, u16, Vec<VerificationType>), // tag == 255, offset_delta, number_of_locals, [_; number_of_locals], number_of_stack_items, [_; number_of_stack_items]
}

impl StackMapFrame {
    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum VerificationType {
    TOP,
    INTEGER,
    NULL,
    UNINITIALIZED_THIS,
    OBJECT,
    UNINITIALIZED,
}

#[derive(Debug, Clone)]
pub(crate) struct VerificationStack {
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

impl VerificationStack {
    pub(crate) fn update(&mut self, inst: &Instruction, constant_pool: &ConstantPool) {
        match inst {
            Instruction::invokespecial(idx) => {
                if let Some(Constant::MethodRef(m)) = constant_pool.get(*idx) {
                    // TODO: Somehow figure out how many parameters are popped when calling the given function
                    todo!();
                    let pop_amount = 1;
                    let l = self.operands.len();
                    self.operands.splice(l - pop_amount..l, []);
                } else {
                    unreachable!();
                }
            }
            Instruction::ldc(idx) => match constant_pool.get(*idx as u16).unwrap() {
                Constant::String(_) => self.operands.push(VerificationType::OBJECT),
                Constant::Integer(_) => self.operands.push(VerificationType::INTEGER),
                Constant::FieldRef(f) => self.operands.push(match f.field.r#type.as_str() {
                    "I" | "Z" | "C" => VerificationType::INTEGER,
                    _ => VerificationType::OBJECT,
                }),
                _ => unreachable!(),
            },
            Instruction::aconst_null => self.operands.push(VerificationType::NULL),
            Instruction::aload_0 | Instruction::aload(_) | Instruction::new(_) => {
                self.operands.push(VerificationType::OBJECT)
            }
            Instruction::iload(_) | Instruction::bipush(_) => {
                self.operands.push(VerificationType::INTEGER)
            }
            Instruction::sipush(_) => self
                .operands
                .extend_from_slice(&[VerificationType::INTEGER, VerificationType::INTEGER]),
            Instruction::ireturn | Instruction::r#return | Instruction::areturn => {
                self.operands.clear()
            }
            Instruction::putfield(_) => {
                self.operands.pop();
                self.operands.pop();
            }
            Instruction::goto(_) | Instruction::ineg => {
                // No changes in stack
            }
            Instruction::istore(_)
            | Instruction::astore(_)
            | Instruction::ifeq(_)
            | Instruction::iflt(_)
            | Instruction::ifge(_)
            | Instruction::ifne(_)
            | Instruction::iadd
            | Instruction::isub
            | Instruction::imul
            | Instruction::idiv
            | Instruction::irem => {
                self.operands.pop();
            }
            Instruction::getfield(idx) => {
                // Use idx to figure out what type the field has
                // pops once and pushes type of field then
                todo!()
            }
            Instruction::dup => {
                let last = self.operands.last().unwrap();
                self.operands.push(last.clone());
            }
            Instruction::relgoto(_)
            | Instruction::reljumpifeq(_)
            | Instruction::reljumpifne(_)
            | Instruction::reljumpiflt(_)
            | Instruction::reljumpifge(_) => unreachable!(),
        }
    }

    pub(crate) fn to_frame(
        &self,
        last_stack: &VerificationStack,
        delta_offset: u16,
    ) -> StackMapFrame {
        // @Nocheckin check whether `==` on vectors is a deep-equality
        if self == last_stack {
            if delta_offset < 64 {
                StackMapFrame::SAME(delta_offset as u8)
            } else {
                StackMapFrame::SAME_EXTENDED(delta_offset)
            }
        } else if self.locals == last_stack.locals
            && self.operands[0..self.operands.len() - 1] == last_stack.operands
        {
            todo!()
        } else {
            dbg!(self.clone(), last_stack.clone());
            todo!()
        }
    }
}
