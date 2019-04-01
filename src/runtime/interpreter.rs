use code::instruction::Instruction;
use runtime::class::{RuntimeClass, ClassTable};
use std::rc::Rc;
use std::cell::RefCell;
use runtime::{Value, IntArray, Object};
use runtime::stack::StackFrame;
use runtime::class::method::{RuntimeMethod, MethodDescriptor};

enum Step {
    Next,
    Jump(i16),
    Return(Value),
    ReturnVoid,
    Throw(Value)
}

pub enum InvokeResult {
    Void,
    Value(Value),
    Exception
}

#[derive(Debug)]
pub enum InterpreterError {
    UnhandledInstruction(Instruction),
    UnexpectedOperand,
    InvalidArrayType
}

pub fn invoke_static_method(arguments: Vec<Value>,
                            method: &RuntimeMethod,
                            class: &Rc<RuntimeClass>,
                            class_table: &ClassTable) -> Option<InvokeResult> {
    let mut locals = arguments.clone();
    let mut stack_frame = StackFrame::new_frame_with_locals(method.code.max_stack, method.code.max_locals, arguments);
    interpret(&mut stack_frame, method, class, class_table)
}

pub fn invoke_virtual_method(this: Rc<RefCell<Object>>,
                             method: &RuntimeMethod,
                             arguments: Vec<Value>,
                             class: &Rc<RuntimeClass>,
                             class_table: &ClassTable) -> Option<InvokeResult> {
    let mut locals = arguments.clone();
    locals.insert(0, Value::ObjectRef(this));
    let mut stack_frame = StackFrame::new_frame_with_locals(method.code.max_stack, method.code.max_locals, locals);
    interpret(&mut stack_frame, method, class, class_table)
}

pub fn interpret(stack_frame: &mut StackFrame,
                 method: &RuntimeMethod,
                 class: &Rc<RuntimeClass>,
                 class_table: &ClassTable) -> Option<InvokeResult> {
    let end_index: u16 = method.code.instructions.len() as u16 - 1;
    let mut current_index: u16 = 0;
    let mut done = false;

    while done == false {
        let tagged_instruction = method.code.instructions.get(current_index as usize).unwrap();
        println!("{}: {:?}", tagged_instruction.index, tagged_instruction.instruction);

        let result = interpret_instruction(
            &tagged_instruction.instruction,
            stack_frame,
            class,
            class_table
        );

        match result {
            Ok(step) => {
                match step {
                    Step::Next => {
                        if current_index == end_index {
                            done = true;
                        } else {
                            current_index += 1;
                        }
                    },
                    Step::Jump(offset) => {
                        let current_code_index = tagged_instruction.index;
                        let branch_code_index = ((current_code_index as i16) + offset) as u16;
                        let next_index = method.code.instructions
                            .iter()
                            .position(|&t| t.index == branch_code_index)
                            .unwrap() as u16;
                        current_index = next_index;
                    },
                    Step::Return(value) => {
                        return Some(InvokeResult::Value(value));
                    },
                    Step::ReturnVoid => {
                        return Some(InvokeResult::Void);
                    },
                    Step::Throw(value) => {
                        // TODO: Search in exception handler table.
                    }
                }
            },
            Err(e) => {
                println!("{:?}", e);
                return None; // TODO: Fix
            }
        }
    }

    println!("{:?}", stack_frame);
    println!("{:?}", std::mem::size_of::<Rc<RefCell<Vec<i32>>>>());

    // TODO: This line of code should never be reached, so we should return a proper error here.
    return None;
}

fn interpret_instruction(instruction: &Instruction, stack_frame: &mut StackFrame, class: &Rc<RuntimeClass>, class_table: &ClassTable) -> Result<Step, InterpreterError> {
    match instruction {
        Instruction::AconstNull => {
            stack_frame.push(Value::Null);
            Ok(Step::Next)
        },
        Instruction::Aload { index } => {
            // It's not necessary to type check perhaps?
            // The typed instructions should really be used for knowing how many bytes to read/write.
            let operand = stack_frame.get_local(*index as usize).clone();
            stack_frame.push(operand);
            Ok(Step::Next)
        },
        Instruction::Aload0 => {
            let operand = stack_frame.get_local(0).clone();
            stack_frame.push(operand);
            Ok(Step::Next)
        },
        Instruction::Aload0 => {
            let operand = stack_frame.get_local(0).clone();
            stack_frame.push(operand);
            Ok(Step::Next)
        },
        Instruction::Aload1 => {
            let operand = stack_frame.get_local(1).clone();
            stack_frame.push(operand);
            Ok(Step::Next)
        },
        Instruction::Aload2 => {
            let operand = stack_frame.get_local(2).clone();
            stack_frame.push(operand);
            Ok(Step::Next)
        },
        Instruction::Aload3 => {
            let operand = stack_frame.get_local(3).clone();
            stack_frame.push(operand);
            Ok(Step::Next)
        },
        Instruction::Astore0 => {
            let operand = stack_frame.pop().unwrap();
            stack_frame.set_local(0, operand);
            Ok(Step::Next)
        },
        Instruction::Astore1 => {
            let operand = stack_frame.pop().unwrap();
            stack_frame.set_local(1, operand);
            Ok(Step::Next)
        },
        Instruction::Astore2 => {
            let operand = stack_frame.pop().unwrap();
            stack_frame.set_local(2, operand);
            Ok(Step::Next)
        },
        Instruction::Astore3 => {
            let operand = stack_frame.pop().unwrap();
            stack_frame.set_local(3, operand);
            Ok(Step::Next)
        },
        Instruction::Bipush { byte } => {
            let value = *byte as i32;
            stack_frame.push_int(value);
            Ok(Step::Next)
        },
        Instruction::Dup => {
            // TODO: Do we need to clone twice here, or is once sufficient?
            // What ends up happening is that we clone it twice and move those out.
            // The original one is freed when we exit the scope.
            let operand = stack_frame.pop().unwrap();
            stack_frame.push(operand.clone());
            stack_frame.push(operand.clone());
            Ok(Step::Next)
        },
        Instruction::Getfield { index } => {
            let object_reference = stack_frame.pop_object_reference()?;
            let field_ref = class.constant_pool.get_field_ref(*index).unwrap();
            let value = object_reference.borrow_mut().get_field(field_ref.name_and_type.name);
            stack_frame.push(value);
            Ok(Step::Next)
        },
        Instruction::Goto { branch_offset } => Ok(Step::Jump(*branch_offset)),
        Instruction::Iadd => {
            let v2 = stack_frame.pop_int()?;
            let v1 = stack_frame.pop_int()?;
            stack_frame.push_int(v1 + v2);
            Ok(Step::Next)
        },
        Instruction::Iaload => {
            let index = stack_frame.pop_int()?;
            let array = stack_frame.pop_int_array()?;
            let value = array.borrow().get(index as usize);
            stack_frame.push_int(value);
            Ok(Step::Next)
        },
        Instruction::Iastore => {
            let value = stack_frame.pop_int()?;
            let index = stack_frame.pop_int()?;
            let mut array = stack_frame.pop_int_array()?;
            array.borrow_mut().set(index as usize, value);
            Ok(Step::Next)
        },
        Instruction::Iconst0 => {
            stack_frame.push_int(0);
            Ok(Step::Next)
        },
        Instruction::Iconst1 => {
            stack_frame.push_int(1);
            Ok(Step::Next)
        },
        Instruction::Iconst2 => {
            stack_frame.push_int(2);
            Ok(Step::Next)
        },
        Instruction::Iconst3 => {
            stack_frame.push_int(3);
            Ok(Step::Next)
        },
        Instruction::Iconst4 => {
            stack_frame.push_int(4);
            Ok(Step::Next)
        },
        Instruction::Iconst5 => {
            stack_frame.push_int(5);
            Ok(Step::Next)
        },
        Instruction::IfIcmpeq { branch_offset } => {
            let value2 = stack_frame.pop_int()?;
            let value1 = stack_frame.pop_int()?;
            if value1 == value2 {
                Ok(Step::Jump(*branch_offset))
            } else {
                Ok(Step::Next)
            }
        },
        Instruction::IfIcmpne { branch_offset } => {
            let value2 = stack_frame.pop_int()?;
            let value1 = stack_frame.pop_int()?;
            if value1 != value2 {
                Ok(Step::Jump(*branch_offset))
            } else {
                Ok(Step::Next)
            }
        },
        Instruction::IfIcmplt { branch_offset } => {
            let value2 = stack_frame.pop_int()?;
            let value1 = stack_frame.pop_int()?;
            if value1 < value2 {
                Ok(Step::Jump(*branch_offset))
            } else {
                Ok(Step::Next)
            }
        },
        Instruction::IfIcmpge { branch_offset } => {
            let value2 = stack_frame.pop_int()?;
            let value1 = stack_frame.pop_int()?;
            if value1 >= value2 {
                Ok(Step::Jump(*branch_offset))
            } else {
                Ok(Step::Next)
            }
        },
        Instruction::IfIcmpgt { branch_offset } => {
            let value2 = stack_frame.pop_int()?;
            let value1 = stack_frame.pop_int()?;
            if value1 > value2 {
                Ok(Step::Jump(*branch_offset))
            } else {
                Ok(Step::Next)
            }
        },
        Instruction::IfIcmple { branch_offset } => {
            let value2 = stack_frame.pop_int()?;
            let value1 = stack_frame.pop_int()?;
            if value1 <= value2 {
                Ok(Step::Jump(*branch_offset))
            } else {
                Ok(Step::Next)
            }
        },
        Instruction::Ifeq { branch_offset } => {
            let value = stack_frame.pop_int()?;
            if value == 0 {
                Ok(Step::Jump(*branch_offset))
            } else {
                Ok(Step::Next)
            }
        },
        Instruction::Ifne { branch_offset } => {
            let value = stack_frame.pop_int()?;
            if value != 0 {
                Ok(Step::Jump(*branch_offset))
            } else {
                Ok(Step::Next)
            }
        },
        Instruction::Iflt { branch_offset } => {
            let value = stack_frame.pop_int()?;
            if value < 0 {
                Ok(Step::Jump(*branch_offset))
            } else {
                Ok(Step::Next)
            }
        },
        Instruction::Ifge { branch_offset } => {
            let value = stack_frame.pop_int()?;
            if value >= 0 {
                Ok(Step::Jump(*branch_offset))
            } else {
                Ok(Step::Next)
            }
        },
        Instruction::Ifgt { branch_offset } => {
            let value = stack_frame.pop_int()?;
            if value > 0 {
                Ok(Step::Jump(*branch_offset))
            } else {
                Ok(Step::Next)
            }
        },
        Instruction::Ifle { branch_offset } => {
            let value = stack_frame.pop_int()?;
            if value <= 0 {
                Ok(Step::Jump(*branch_offset))
            } else {
                Ok(Step::Next)
            }
        },
        Instruction::Ifnonnull { branch_offset} => {
            let reference = stack_frame.pop().unwrap();

            match reference {
                Value::Null => Ok(Step::Next),
                _ => Ok(Step::Jump(*branch_offset))
            }
        },
        Instruction::Ifnull { branch_offset} => {
            let reference = stack_frame.pop().unwrap();

            match reference {
                Value::Null => Ok(Step::Jump(*branch_offset)),
                _ => Ok(Step::Next)
            }
        },
        Instruction::Iinc { index, constant } => {
            let local = stack_frame.get_int_local(*index as usize)?;
            stack_frame.set_int_local(*index as usize, local + (*constant as i32));
            Ok(Step::Next)
        },
        Instruction::Iload { index } => {
            let int = stack_frame.get_int_local(*index as usize)?;
            stack_frame.push_int(int);
            Ok(Step::Next)
        },
        Instruction::Iload0 => {
            let int = stack_frame.get_int_local(0)?;
            stack_frame.push_int(int);
            Ok(Step::Next)
        },
        Instruction::Iload1 => {
            let int = stack_frame.get_int_local(1)?;
            stack_frame.push_int(int);
            Ok(Step::Next)
        },
        Instruction::Iload2 => {
            let int = stack_frame.get_int_local(2)?;
            stack_frame.push_int(int);
            Ok(Step::Next)
        },
        Instruction::Iload3 => {
            let int = stack_frame.get_int_local(3)?;
            stack_frame.push_int(int);
            Ok(Step::Next)
        },
        Instruction::Invokespecial { index } => {
            // TODO: Implement, will need to read method descriptor to determine how many operands to pull off?
            // TODO: Remove
            stack_frame.pop();
            stack_frame.pop();
            Ok(Step::Next)
        },
        Instruction::Invokevirtual { index } => {
            let method_ref = class.constant_pool.get_method_ref(*index).unwrap();
            let invoked_class = class_table.get_class(method_ref.class_name.as_str()).unwrap();

            // TODO: Verify access flags

            let method = invoked_class.get_method(method_ref.name_and_type.name.as_str()).unwrap();
            let method_descriptor = MethodDescriptor::parse(method_ref.name_and_type.descriptor.as_str()).unwrap();

            let mut arguments = stack_frame.pop_many(method_descriptor.parameters_length()).unwrap();
            arguments.reverse();
            let object = stack_frame.pop_object_reference()?;

            let invoke_result = invoke_virtual_method(object, method, arguments, invoked_class, class_table).unwrap();
            match invoke_result {
                InvokeResult::Value(value) => {
                    stack_frame.push(value);
                    Ok(Step::Next)
                },
                InvokeResult::Void => {
                    Ok(Step::Next)
                },
                InvokeResult::Exception => {
                    Ok(Step::Next)
                }
            }
        },
        Instruction::Imul => {
            let v2 = stack_frame.pop_int()?;
            let v1 = stack_frame.pop_int()?;
            stack_frame.push_int(v1 * v2);
            Ok(Step::Next)
        },
        Instruction::Ireturn => {
            // TODO: We should probably validate the operand type.
            let value = stack_frame.pop().unwrap();
            Ok(Step::Return(value))
        },
        Instruction::Istore(index) => {
            let int = stack_frame.pop_int()?;
            stack_frame.set_int_local(*index as usize, int);
            Ok(Step::Next)
        },
        Instruction::Istore0 => {
            let int = stack_frame.pop_int()?;
            stack_frame.set_int_local(0, int);
            Ok(Step::Next)
        },
        Instruction::Istore1 => {
            let int = stack_frame.pop_int()?;
            stack_frame.set_int_local(1, int);
            Ok(Step::Next)
        },
        Instruction::Istore2 => {
            let int = stack_frame.pop_int()?;
            stack_frame.set_int_local(2, int);
            Ok(Step::Next)
        },
        Instruction::Istore3 => {
            let int = stack_frame.pop_int()?;
            stack_frame.set_int_local(3, int);
            Ok(Step::Next)
        },
        Instruction::Isub => {
            let value2 = stack_frame.pop_int()?;
            let value1 = stack_frame.pop_int()?;
            stack_frame.push_int(value1 - value2);
            Ok(Step::Next)
        },
        Instruction::New { index } => {
            let class_name = class.constant_pool.get_class_name(*index).unwrap();
            let runtime_class = class_table.get_class(&*class_name).unwrap();
            let memory: Vec<Value> = runtime_class.default_fields();

            let object = Object {
                class: runtime_class.clone(),
                memory
            };

            let object_reference = Value::ObjectRef(Rc::new(RefCell::new(object)));
            stack_frame.push(object_reference);

            Ok(Step::Next)
        },
        Instruction::Newarray { atype } => {
            let count = stack_frame.pop_int()?;
            // TODO: These are array type codes. We could classify them.
            match atype {
                10 => {
                    let array = Value::IntegerArrayRef(IntArray::new(count as usize));
                    stack_frame.push(array);
                    Ok(Step::Next)
                },
                _ => Err(InterpreterError::InvalidArrayType)
            }
        },
        Instruction::Pop => {
            stack_frame.pop();
            Ok(Step::Next)
        },
        Instruction::Putfield { index } => {
            let value = stack_frame.pop().unwrap();
            let object_reference = stack_frame.pop_object_reference()?;
            let field_ref = class.constant_pool.get_field_ref(*index).unwrap();
            object_reference.borrow_mut().put_field(field_ref.name_and_type.name, value);
            Ok(Step::Next)
        },
        Instruction::Sipush(value) => {
            stack_frame.push_int(*value);
            Ok(Step::Next)
        },
        Instruction::Return => {
            Ok(Step::ReturnVoid)
        },
        x => Err(InterpreterError::UnhandledInstruction(*x))
    }
}
