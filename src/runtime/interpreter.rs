use code::instruction::Instruction;
use class::{ConstantPool, Method};
use code::RuntimeMethod;

// TODO: Implement locals and stack with an array
#[derive(Debug)]
struct StackFrame {
    locals: Vec<StackVariable>,
    stack: Vec<StackVariable>
}

impl StackFrame {

    fn pop_stack(&mut self) -> Option<StackVariable> {
        self.stack.pop()
    }

    fn push_stack(&mut self, operand: StackVariable) {
        self.stack.push(operand)
    }

    fn get_local(&self, index: usize) -> StackVariable {
        self.locals[index]
    }

    fn set_local(&mut self, index: usize, var: StackVariable) {
        self.locals[index] = var
    }

    fn new_frame(max_stack: u16, max_locals: u16) -> StackFrame {
        let locals: Vec<StackVariable> = vec![StackVariable::Empty; max_locals as usize];
        let stack: Vec<StackVariable> = Vec::new();

        StackFrame { locals, stack }
    }

}

#[derive(Copy, Clone, Debug)]
enum StackVariable {
    Long(i64),
    Integer(i32),
    Short(i16),
    Byte(i8),
    Character(char),
    Empty
}

pub fn interpret(method: &RuntimeMethod, cp: &ConstantPool) {
    let mut stack: Vec<StackFrame> = Vec::new();

    let mut stack_frame = StackFrame::new_frame(method.max_stack, method.max_locals);

    for instruction in method.code.iter() {
        interpret_instruction(instruction, &mut stack_frame);
    }

    println!("{:?}", stack_frame);
}

fn interpret_instruction(instruction: &Instruction, stack_frame: &mut StackFrame) {
    println!("{:?}", instruction);

    match instruction {
        Instruction::Iadd => {
            let operand_1 = stack_frame.pop_stack().unwrap();
            let operand_2 = stack_frame.pop_stack().unwrap();

            match operand_1 {
                StackVariable::Integer(i1) => {
                    match operand_2 {
                        StackVariable::Integer(i2) => {
                            stack_frame.push_stack(StackVariable::Integer(i2 + i1))
                        },
                        _ => {
                            panic!("Found non-integer on top of operand stack.");
                        }
                    }
                },
                _ => {
                    panic!("Found non-integer on top of operand stack.");
                }
            }
        },
        Instruction::Iconst0 => {
            stack_frame.push_stack(StackVariable::Integer(0));
        },
        Instruction::Iconst1 => {
            stack_frame.push_stack(StackVariable::Integer(1));
        },
        Instruction::Iconst2 => {
            stack_frame.push_stack(StackVariable::Integer(2));
        },
        Instruction::Iconst3 => {
            stack_frame.push_stack(StackVariable::Integer(3));
        },
        Instruction::Iconst4 => {
            stack_frame.push_stack(StackVariable::Integer(4));
        },
        Instruction::Iconst5 => {
            stack_frame.push_stack(StackVariable::Integer(5));
        },
        Instruction::Iload1 => {
            let operand = stack_frame.get_local(1);

            match operand {
                i @ StackVariable::Integer { .. } => {
                    stack_frame.push_stack(i);
                },
                _ => {
                    panic!("Found non-integer on top of operand stack.");
                }
            }
        },
        Instruction::Iload2 => {
            let operand = stack_frame.get_local(2);

            match operand {
                i @ StackVariable::Integer { .. } => {
                    stack_frame.push_stack(i);
                },
                _ => {
                    panic!("Found non-integer on top of operand stack.");
                }
            }
        },

        Instruction::Iload3 => {
            let operand = stack_frame.get_local(3);

            match operand {
                i @ StackVariable::Integer { .. } => {
                    stack_frame.push_stack(i);
                },
                _ => {
                    panic!("Found non-integer on top of operand stack.");
                }
            }
        },
        Instruction::Istore(index) => {
            let operand = stack_frame.pop_stack().unwrap();

            match operand {
                i @ StackVariable::Integer { .. } => {
                    stack_frame.set_local(*index as usize, i);
                },
                _ => {
                    panic!("Found non-integer on top of operand stack.");
                }
            }
        },
        Instruction::Istore1 => {
            let operand = stack_frame.pop_stack().unwrap();

            match operand {
                i @ StackVariable::Integer { .. } => {
                    stack_frame.set_local(1, i);
                },
                _ => {
                    panic!("Found non-integer on top of operand stack.");
                }
            }
        },
        Instruction::Istore2 => {
            let operand = stack_frame.pop_stack().unwrap();

            match operand {
                i @ StackVariable::Integer { .. } => {
                    stack_frame.set_local(2, i);
                },
                _ => {
                    panic!("Found non-integer on top of operand stack.");
                }
            }
        },
        Instruction::Istore3 => {
            let operand = stack_frame.pop_stack().unwrap();

            match operand {
                i @ StackVariable::Integer { .. } => {
                    stack_frame.set_local(3, i);
                },
                _ => {
                    panic!("Found non-integer on top of operand stack.");
                }
            }
        },
        Instruction::Isub => {
            let operand_1 = stack_frame.pop_stack().unwrap();
            let operand_2 = stack_frame.pop_stack().unwrap();

            match operand_1 {
                StackVariable::Integer(i1) => {
                    match operand_2 {
                        StackVariable::Integer(i2) => {
                            stack_frame.push_stack(StackVariable::Integer(i2 - i1))
                        },
                        _ => {
                            panic!("Found non-integer on top of operand stack.");
                        }
                    }
                },
                _ => {
                    panic!("Found non-integer on top of operand stack.");
                }
            }
        },
        _ => {}
    }
}
