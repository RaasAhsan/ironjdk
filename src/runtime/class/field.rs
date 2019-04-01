use class::{Field, ConstantPool};

#[derive(Debug)]
pub struct RuntimeField {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: FieldDescriptor
}

impl RuntimeField {

    pub fn from_class_field(field: &Field, cp: &ConstantPool) -> Option<RuntimeField> {
        let name = cp.get_utf8(field.name_index).unwrap();
        let descriptor_tag = cp.get_utf8(field.descriptor_index).unwrap();

        // TODO: We can do this decoding in the class representation
        let descriptor = FieldDescriptor::from_str(descriptor_tag.as_str()).unwrap();

        let runtime_field = RuntimeField {
            access_flags: field.access_flags,
            name,
            descriptor
        };

        Some(runtime_field)
    }

}

// TODO: See $4.3.2 of JVMS for a more precise encoding.
#[derive(Debug)]
pub enum FieldDescriptor {
    Byte,
    Character,
    Double,
    Float,
    Integer,
    Long,
    ClassReference { class_name: String },
    Short,
    Boolean,
    ArrayReference()
}

impl FieldDescriptor {

    pub fn from_str(s: &str) -> Option<FieldDescriptor> {
        match s {
            "B" => Some(FieldDescriptor::Byte),
            "C" => Some(FieldDescriptor::Character),
            "D" => Some(FieldDescriptor::Double),
            "F" => Some(FieldDescriptor::Float),
            "I" => Some(FieldDescriptor::Integer),
            "J" => Some(FieldDescriptor::Long),
            x if x.starts_with("L") && x.ends_with(";") => {
                let class_name = String::from(&x[1..(x.len()-1)]);
                Some(FieldDescriptor::ClassReference { class_name })
            },
            "S" => Some(FieldDescriptor::Short),
            "Z" => Some(FieldDescriptor::Boolean),
            // TODO: Parse array types
            _ => None
        }
    }

}
