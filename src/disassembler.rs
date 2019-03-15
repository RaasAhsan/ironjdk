use instruction::Instruction;

const AALOAD: u8 = 0x32;
const AASTORE: u8 = 0x53;
const ACONST_NULL: u8 = 0x01;
const ALOAD_0: u8 = 0x2a;
const ALOAD_1: u8 = 0x2b;
const ALOAD_2: u8 = 0x2c;
const ALOAD_3: u8 = 0x2d;
const ANEWARRAY: u8 = 0xbd;
const ARETURN: u8 = 0xb0;
const ARRAYLENGTH: u8 = 0xbe;
const ASTORE: u8 = 0x3a;
const ASTORE_0: u8 = 0x4b;
const ASTORE_1: u8 = 0x4c;
const ASTORE_2: u8 = 0x4d;
const ASTORE_3: u8 = 0x4e;
const ATHROW: u8 = 0xbf;
const BALOAD: u8 = 0x33;
const BASTORE: u8 = 0x54;
const BIPUSH: u8 = 0x10;
const CALOAD: u8 = 0x34;
const CASTORE: u8 = 0x55;
const CHECKCAST: u8 = 0xc0;
const D2F: u8 = 0x90;
const D2I: u8 = 0x8e;
const D2L: u8 = 0x8f;
const DADD: u8 = 0x63;
const DALOAD: u8 = 0x31;
const DCMPG: u8 = 0x98;
const DCMPL: u8 = 0x97;
const DCONST_0: u8 = 0x0e;
const DCONST_1: u8 = 0x0f;
const DDIV: u8 = 0x6f;
const DLOAD: u8 = 0x18;
const DLOAD_0: u8 = 0x26;
const DLOAD_1: u8 = 0x27;
const DLOAD_2: u8 = 0x28;
const DLOAD_3: u8 = 0x29;
const DMUL: u8 = 0x6b;
const DNEG: u8 = 0x77;
const DREM: u8 = 0x73;
const DRETURN: u8 = 0xaf;
const DSTORE: u8 = 0x39;
const DSTORE_0: u8 = 0x47;
const DSTORE_1: u8 = 0x48;
const DSTORE_2: u8 = 0x49;
const DSTORE_3: u8 = 0x4a;
const DSUB: u8 = 0x67;
const DUP: u8 = 0x59;
const DUP_X1: u8 = 0x5a;
const DUP_X2: u8 = 0x5b;
const DUP2: u8 = 0x5c;
const DUP2_X1: u8 = 0x5d;
const DUP2_X2: u8 = 0x5e;
const F2D: u8 = 0x8d;
const F2I: u8 = 0x8b;
const F2L: u8 = 0x8c;
const FADD: u8 = 0x62;
const FALOAD: u8 = 0x30;
const FASTORE: u8 = 0x51;
const FCMPG: u8 = 0x96;
const FCMPL: u8 = 0x95;
const FCONST_0: u8 = 0x0b;
const FCONST_1: u8 = 0x0c;
const FCONST_2: u8 = 0x0d;
const FDIV: u8 = 0x6e;
const FLOAD: u8 = 0x17;
const FLOAD_0: u8 = 0x22;
const FLOAD_1: u8 = 0x23;
const FLOAD_2: u8 = 0x24;
const FLOAD_3: u8 = 0x25;
const FMUL: u8 = 0x6a;
const FNEG: u8 = 0x76;
const FREM: u8 = 0x72;
const FRETURN: u8 = 0xae;
const FSTORE: u8 = 0x38;
const FSTORE_0: u8 = 0x43;
const FSTORE_1: u8 = 0x44;
const FSTORE_2: u8 = 0x45;
const FSTORE_3: u8 = 0x46;
const FSUB: u8 = 0x66;
const GETFIELD: u8 = 0xb4;
const GETSTATIC: u8 = 0xb2;
const GOTO: u8 = 0xa7;
const GOTO_W: u8 = 0xc8;
const I2B: u8 = 0x91;
const I2C: u8 = 0x92;
const I2D: u8 = 0x87;
const I2F: u8 = 0x86;
const I2L: u8 = 0x85;
const I2S: u8 = 0x93;
const IADD: u8 = 0x60;
const IALOAD: u8 = 0x2e;
const IAND: u8 = 0x7e;
const IASTORE: u8 = 0x4f;
const ICONST_M1: u8 = 0x02;
const ICONST_0: u8 = 0x03;
const ICONST_1: u8 = 0x04;
const ICONST_2: u8 = 0x05;
const ICONST_3: u8 = 0x06;
const ICONST_4: u8 = 0x07;
const ICONST_5: u8 = 0x08;
const IDIV: u8 = 0x6c;
const IF_ACMPEQ: u8 = 0xa5;
const IF_ACMPNE: u8 = 0xa6;
const IF_ICMPEQ: u8 = 0x9f;
const IF_ICMPNE: u8 = 0xa0;
const IF_ICMPLT: u8 = 0xa1;
const IF_ICMPGE: u8 = 0xa2;
const IF_ICMPGT: u8 = 0xa3;
const IF_ICMPLE: u8 = 0xa4;
const IFEQ: u8 = 0x99;
const IFNE: u8 = 0x9a;
const IFLT: u8 = 0x9b;
const IFGE: u8 = 0x9c;
const IFGT: u8 = 0x9d;
const IFLE: u8 = 0x9e;
const IFNONNULL: u8 = 0xc7;
const IFNULL: u8 = 0xc6;
const IINC: u8 = 0x84;
const ILOAD: u8 = 0x15;
const ILOAD_0: u8 = 0x1a;
const ILOAD_1: u8 = 0x1b;
const ILOAD_2: u8 = 0x1c;
const ILOAD_3: u8 = 0x1d;
const IMUL: u8 = 0x68;
const INEG: u8 = 0x74;
const INSTANCEOF: u8 = 0xc1;
const INVOKEDYNAMIC: u8 = 0xba;
const INVOKEINTERFACE: u8 = 0xb9;
const INVOKESPECIAL: u8 = 0xb7;
const INVOKESTATIC: u8 = 0xb8;
const INVOKEVIRTUAL: u8 = 0xb6;
const IOR: u8 = 0x80;
const IREM: u8 = 0x70;
const IRETURN: u8 = 0xac;
const ISHL: u8 = 0x78;
const ISHR: u8 = 0x7a;
const ISTORE: u8 = 0x36;
const ISTORE_0: u8 = 0x3b;
const ISTORE_1: u8 = 0x3c;
const ISTORE_2: u8 = 0x3d;
const ISTORE_3: u8 = 0x3e;
const ISUB: u8 = 0x64;
const IUSHR: u8 = 0x7c;
const IXOR: u8 = 0x82;
const JSR: u8 = 0xa8;
const JSR_W: u8 = 0xc9;
const L2D: u8 = 0x8a;
const L2F: u8 = 0x89;
const L2I: u8 = 0x88;
const LADD: u8 = 0x61;
const LALOAD: u8 = 0x2f;
const LAND: u8 = 0x7f;
const LASTORE: u8 = 0x50;
const LCMP: u8 = 0x94;
const LCONST_0: u8 = 0x09;
const LCONST_1: u8 = 0x0a;
const LDC: u8 = 0x12;
const LDC_W: u8 = 0x13;
const LDC2_W: u8 = 0x14;
const LDIV: u8 = 0x6d;
const LLOAD: u8 = 0x16;
const LLOAD_0: u8 = 0x1e;
const LLOAD_1: u8 = 0x1f;
const LLOAD_2: u8 = 0x20;
const LLOAD_3: u8 = 0x21;
const LMUL: u8 = 0x69;
const LNEG: u8 = 0x75;
const LOOKUPSWITCH: u8 = 0xab;
const LOR: u8 = 0x81;
const LREM: u8 = 0x71;
const LRETURN: u8 = 0xad;
const LSHL: u8 = 0x79;
const LSHR: u8 = 0x7b;
const LSTORE: u8 = 0x37;
const LSTORE_0: u8 = 0x3f;
const LSTORE_1: u8 = 0x40;
const LSTORE_2: u8 = 0x41;
const LSTORE_3: u8 = 0x42;
const LSUB: u8 = 0x65;
const LUSHR: u8 = 0x7d;
const LXOR: u8 = 0x83;
const MONITORENTER: u8 = 0xc2;
const MONITOREXIT: u8 = 0xc3;
const MULTIANEWARRAY: u8 = 0xc5;
const NEW: u8 = 0xbb;
const NEWARRAY: u8 = 0xbc;
const NOP: u8 = 0x00;
const POP: u8 = 0x57;
const POP2: u8 = 0x58;
const PUTFIELD: u8 = 0xb5;
const PUTSTATIC: u8 = 0xb3;
const RET: u8 = 0xa9;
const RETURN: u8 = 0xb1;
const SALOAD: u8 = 0x35;
const SASTORE: u8 = 0x56;
const SIPUSH: u8 = 0x11;
const SWAP: u8 = 0x5f;
const TABLESWITCH: u8 = 0xaa;
const WIDE: u8 = 0xc4;



enum DisassemblerError {
    EndOfCode,
    InvalidOpcode(u8)
}

pub fn disassemble_code(bytes: &mut Vec<u8>) -> Result<Vec<Instruction>, DisassemblerError> {

}

fn parse_instruction(bytes: &mut Vec<u8>) -> Result<Instruction, DisassemblerError> {
    let opcode = parse_u8(bytes)?;

    match opcode {
        AALOAD=> Ok(Instruction::Aaload),
        AASTORE => Ok(Instruction::Aastore),
        ACONST_NULL => Ok(Instruction::AconstNull),
        ALOAD_0 => Ok(Instruction::Aload0),
        ALOAD_1 => Ok(Instruction::Aload1),
        ALOAD_2 => Ok(Instruction::Aload2),
        ALOAD_3 => Ok(Instruction::Aload3),
        ANEWARRAY => {
            let indexbyte1 = parse_u8(bytes)?;
            let indexbyte2 = parse_u8(bytes)?;

            Ok(Instruction::Anewarray { indexbyte1, indexbyte2 })
        },
        ARETURN => Ok(Instruction::Areturn),
        ARRAYLENGTH => Ok(Instruction::Arraylength),
        ASTORE => {
            let index = parse_u8(bytes)?;

            Ok(Instruction::Astore { index })
        }
        ASTORE_0 => Ok(Instruction::Astore0),
        ASTORE_1 => Ok(Instruction::Astore1),
        ASTORE_2 => Ok(Instruction::Astore2),
        ASTORE_3 => Ok(Instruction::Astore3),
        ATHROW => Ok(Instruction::Athrow),
        BALOAD => Ok(Instruction::Baload),
        BASTORE => Ok(Instruction::Bastore),
        BIPUSH => {
            let byte = parse_u8(bytes)? as i8;

            Ok(Instruction::Bipush { byte })
        }
        x => Err(DisassemblerError::InvalidOpcode(x))
    }
}

fn parse_u8(bytes: &mut Vec<u8>) -> Result<u8, DisassemblerError> {
    match buffer.get(0) {
        Some(&byte) => {
            buffer.remove(0);
            Ok(byte)
        },
        None => Err(DisassemblerError::EndOfCode)
    }
}