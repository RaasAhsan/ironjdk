use class::{Method, Attribute, ConstantPool, Field};
use code::disassembler;
use code::instruction::TaggedInstruction;

#[derive(Debug)]
pub struct RuntimeMethod {
    pub name: String,
    pub access_flags: u16,
    pub code: Code
}

impl RuntimeMethod {

    pub fn from_class_method(method: &Method, cp: &ConstantPool) -> Option<RuntimeMethod> {
        let name = cp.get_utf8(method.name_index).unwrap();
        let code = RuntimeMethod::get_code(method)?;

        let runtime_method = RuntimeMethod {
            name,
            access_flags: method.access_flags,
            code
        };

        Some(runtime_method)
    }

    fn get_code(method: &Method) -> Option<Code> {
        for a in method.attributes.iter() {
            match a {
                &Attribute::Code { max_stack, max_locals, ref code, .. } => {
                    let instructions = disassembler::disassemble_code(&code).ok()?;

                    let code = Code {
                        max_stack,
                        max_locals,
                        instructions
                    };

                    return Some(code);
                },
                _ => {}
            }
        }

        None
    }

}

#[derive(Debug)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub instructions: Vec<TaggedInstruction>
}

// Method descriptors are described in JVMS $4.3.3
#[derive(Debug)]
pub struct MethodDescriptor {
    parameter_descriptors: Vec<FieldType>,
    return_descriptor: ReturnDescriptor
}

impl MethodDescriptor {

    pub fn parameters_length(&self) -> usize {
        self.parameter_descriptors.len()
    }

    pub fn parse(input: &str) -> Option<MethodDescriptor> {
        let mut lexemes = MethodDescriptor::lex(input);
        MethodDescriptor::parse_method_descriptor(&mut lexemes).ok()
    }

    fn lex(input: &str) -> Vec<Lexeme> {
        let mut lexemes = Vec::new();
        let mut remaining = String::from(input);

        while remaining.len() > 0 {
            let lexeme = match &remaining[0..1] {
                "(" => Lexeme::LeftParentheses,
                ")" => Lexeme::RightParentheses,
                "B" => Lexeme::Byte,
                "C" => Lexeme::Character,
                "D" => Lexeme::Double,
                "F" => Lexeme::Float,
                "I" => Lexeme::Integer,
                "J" => Lexeme::Long,
                "S" => Lexeme::Short,
                "[" => Lexeme::LeftSquareBracket,
                "V" => Lexeme::Void,
                _ => {
                    if remaining.starts_with("L") {
                        let end = remaining.find(';').unwrap();
                        let class_name = String::from(&remaining[1..end]);
                        Lexeme::Class(class_name)
                    } else {
                        panic!("invalid method descriptor found");
                    }
                }
            };
            remaining = String::from(&remaining[(lexeme.length())..]);
            lexemes.push(lexeme);
        };

        lexemes
    }

    fn parse_method_descriptor(lexemes: &mut Vec<Lexeme>) -> Result<MethodDescriptor, String> {
        // TODO: Would these be better as methods on some struct?
        MethodDescriptor::parse_left_parentheses(lexemes)?;
        let parameter_descriptors = MethodDescriptor::parse_parameter_descriptors(lexemes)?;
        MethodDescriptor::parse_right_parentheses(lexemes)?;
        let return_descriptor = MethodDescriptor::parse_return_descriptor(lexemes)?;

        let method_descriptor = MethodDescriptor {
            parameter_descriptors,
            return_descriptor
        };

        Ok(method_descriptor)
    }

    fn parse_parameter_descriptors(lexemes: &mut Vec<Lexeme>) -> Result<Vec<FieldType>, String> {
        let mut parameter_descriptors: Vec<FieldType> = Vec::new();

        let mut done = false;
        while !done {
            match MethodDescriptor::parse_field_type(lexemes) {
                Ok(field_type) => parameter_descriptors.push(field_type),
                Err(e) => done = true
            }
        }

        Ok(parameter_descriptors)
    }

    fn parse_return_descriptor(lexemes: &mut Vec<Lexeme>) -> Result<ReturnDescriptor, String> {
        match MethodDescriptor::parse_field_type(lexemes) {
            Ok(field_type) => Ok(ReturnDescriptor::Field(field_type)),
            Err(e1) => match MethodDescriptor::parse_void(lexemes) {
                Ok(_) => Ok(ReturnDescriptor::Void),
                Err(e2) => Err(e2)
            }
        }
    }

    fn parse_field_type(lexemes: &mut Vec<Lexeme>) -> Result<FieldType, String> {
        let token = match lexemes.get(0) {
            Some(Lexeme::Byte) => Ok(FieldType::Byte),
            Some(Lexeme::Character) => Ok(FieldType::Character),
            Some(Lexeme::Double) => Ok(FieldType::Double),
            Some(Lexeme::Float) => Ok(FieldType::Float),
            Some(Lexeme::Integer) => Ok(FieldType::Integer),
            Some(Lexeme::Long) => Ok(FieldType::Long),
            Some(Lexeme::Class(name)) => Ok(FieldType::Class(name.clone())),
            Some(Lexeme::Short) => Ok(FieldType::Short),
            Some(Lexeme::Boolean) => Ok(FieldType::Boolean),
            // TODO: Array descriptor
            _ => Err(String::from("Did not find field type"))
        };

        match token {
            Ok(_) => {
                lexemes.remove(0);
            },
            Err(_) => {}
        };

        token
    }

    fn parse_left_parentheses(lexemes: &mut Vec<Lexeme>) -> Result<(), String> {
        match lexemes.get(0) {
            Some(Lexeme::LeftParentheses) => {
                lexemes.remove(0);
                Ok(())
            },
            _ => Err(String::from("Did not find left parentheses"))
        }
    }

    fn parse_right_parentheses(lexemes: &mut Vec<Lexeme>) -> Result<(), String> {
        match lexemes.get(0) {
            Some(Lexeme::RightParentheses) => {
                lexemes.remove(0);
                Ok(())
            },
            _ => Err(String::from("Did not find right parentheses"))
        }
    }

    fn parse_void(lexemes: &mut Vec<Lexeme>) -> Result<(), String> {
        match lexemes.get(0) {
            Some(Lexeme::Void) => {
                lexemes.remove(0);
                Ok(())
            },
            _ => Err(String::from("Did not find void"))
        }
    }

}

#[derive(Debug)]
enum ReturnDescriptor {
    Void,
    Field(FieldType)
}

#[derive(Debug)]
enum FieldType {
    Byte,
    Character,
    Double,
    Float,
    Integer,
    Long,
    Class(String),
    Short,
    Boolean,
    Array()
}

#[derive(Debug)]
enum Lexeme {
    LeftParentheses,
    RightParentheses,
    Byte,
    Character,
    Double,
    Float,
    Integer,
    Long,
    Class(String),
    Short,
    Boolean,
    LeftSquareBracket,
    Void
}

impl Lexeme {
    fn length(&self) -> usize {
        match self {
            Lexeme::LeftParentheses => 1,
            Lexeme::RightParentheses => 1,
            Lexeme::Byte => 1,
            Lexeme::Character => 1,
            Lexeme::Double => 1,
            Lexeme::Float => 1,
            Lexeme::Integer => 1,
            Lexeme::Long => 1,
            Lexeme::Class(name) => name.len() + 2,
            Lexeme::Short => 1,
            Lexeme::Boolean => 1,
            Lexeme::LeftSquareBracket => 1,
            Lexeme::Void => 1
        }
    }
}
