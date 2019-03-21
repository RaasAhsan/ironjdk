use code::instruction::Instruction;

const AALOAD: u8 = 0x32;
const AASTORE: u8 = 0x53;
const ACONST_NULL: u8 = 0x01;
const ALOAD: u8 = 0x19;
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
const DASTORE: u8 = 0x52;
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

#[derive(Debug)]
pub enum DisassemblerError {
    EndOfCode,
    InvalidOpcode(u8)
}

pub fn disassemble_code(buffer: &Vec<u8>) -> Result<Vec<Instruction>, DisassemblerError> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut bytes: Vec<u8> = buffer.clone();

    while bytes.len() > 0 {
        let i = parse_instruction(&mut bytes)?;
        instructions.push(i);
    }

    Ok(instructions)
}

fn parse_instruction(bytes: &mut Vec<u8>) -> Result<Instruction, DisassemblerError> {
    let opcode = read_u8(bytes)?;

    match opcode {
        x if x == AALOAD => Ok(Instruction::Aaload),
        x if x == AASTORE => Ok(Instruction::Aastore),
        x if x == ACONST_NULL => Ok(Instruction::AconstNull),
        x if x == ALOAD => {
            let index = read_u8(bytes)?;

            Ok(Instruction::Aload { index })
        },
        x if x == ALOAD_0 => Ok(Instruction::Aload0),
        x if x == ALOAD_1 => Ok(Instruction::Aload1),
        x if x == ALOAD_2 => Ok(Instruction::Aload2),
        x if x == ALOAD_3 => Ok(Instruction::Aload3),
        x if x == ANEWARRAY => {
            let index = read_u16(bytes)?;

            Ok(Instruction::Anewarray { index })
        },
        x if x == ARETURN => Ok(Instruction::Areturn),
        x if x == ARRAYLENGTH => Ok(Instruction::Arraylength),
        x if x == ASTORE => {
            let index = read_u8(bytes)?;

            Ok(Instruction::Astore { index })
        }
        x if x == ASTORE_0 => Ok(Instruction::Astore0),
        x if x == ASTORE_1 => Ok(Instruction::Astore1),
        x if x == ASTORE_2 => Ok(Instruction::Astore2),
        x if x == ASTORE_3 => Ok(Instruction::Astore3),
        x if x == ATHROW => Ok(Instruction::Athrow),
        x if x == BALOAD => Ok(Instruction::Baload),
        x if x == BASTORE => Ok(Instruction::Bastore),
        x if x == BIPUSH => {
            let byte = read_u8(bytes)? as i8;

            Ok(Instruction::Bipush { byte })
        }
        x if x == CALOAD => Ok(Instruction::Caload),
        x if x == CASTORE => Ok(Instruction::Castore),
        x if x == CHECKCAST => {
            let index = read_u16(bytes)?;

            Ok(Instruction::Checkcast { index })
        },
        x if x == D2F => Ok(Instruction::D2f),
        x if x == D2I => Ok(Instruction::D2i),
        x if x == D2L => Ok(Instruction::D2l),
        x if x == DADD => Ok(Instruction::Dadd),
        x if x == DALOAD => Ok(Instruction::Daload),
        x if x == DASTORE => Ok(Instruction::Dastore),
        x if x == DCMPG => Ok(Instruction::Dcmpg),
        x if x == DCMPL => Ok(Instruction::Dcmpl),
        x if x == DCONST_0 => Ok(Instruction::Dconst0),
        x if x == DCONST_1 => Ok(Instruction::Dconst1),
        x if x == DDIV => Ok(Instruction::Ddiv),
        x if x == DLOAD => {
            let index = read_u8(bytes)?;

            Ok(Instruction::Dload { index })
        },
        x if x == DLOAD_0 => Ok(Instruction::Dload0),
        x if x == DLOAD_1 => Ok(Instruction::Dload1),
        x if x == DLOAD_2 => Ok(Instruction::Dload2),
        x if x == DLOAD_3 => Ok(Instruction::Dload3),
        x if x == DMUL => Ok(Instruction::Dmul),
        x if x == DNEG => Ok(Instruction::Dneg),
        x if x == DREM => Ok(Instruction::Drem),
        x if x == DRETURN=> Ok(Instruction::Dreturn),
        x if x == DSTORE => {
            let index = read_u8(bytes)?;

            Ok(Instruction::Dstore { index })
        },
        x if x == DSTORE_0 => Ok(Instruction::Dstore0),
        x if x == DSTORE_1 => Ok(Instruction::Dstore1),
        x if x == DSTORE_2 => Ok(Instruction::Dstore2),
        x if x == DSTORE_3 => Ok(Instruction::Dstore3),
        x if x == DSUB => Ok(Instruction::Dsub),
        x if x == DUP => Ok(Instruction::Dup),
        x if x == DUP_X1 => Ok(Instruction::DupX1),
        x if x == DUP_X2 => Ok(Instruction::DupX2),
        x if x == DUP2 => Ok(Instruction::Dup2),
        x if x == DUP2_X1=> Ok(Instruction::Dup2X1),
        x if x == DUP2_X2 => Ok(Instruction::Dup2X2),
        x if x == F2D => Ok(Instruction::F2d),
        x if x == F2I => Ok(Instruction::F2i),
        x if x == F2L => Ok(Instruction::F2l),
        x if x == FADD => Ok(Instruction::Fadd),
        x if x == FALOAD => Ok(Instruction::Faload),
        x if x == FASTORE => Ok(Instruction::Fastore),
        x if x == FCMPG => Ok(Instruction::Fcmpg),
        x if x == FCMPL => Ok(Instruction::Fcmpl),
        x if x == FCONST_0 => Ok(Instruction::Fconst0),
        x if x == FCONST_1 => Ok(Instruction::Fconst1),
        x if x == FCONST_2 => Ok(Instruction::Fconst2),
        x if x == FDIV => Ok(Instruction::Fdiv),
        x if x == FLOAD => {
            let index = read_u8(bytes)?;

            Ok(Instruction::Fload { index })
        },
        x if x == FLOAD_0 => Ok(Instruction::Fload0),
        x if x == FLOAD_1 => Ok(Instruction::Fload1),
        x if x == FLOAD_2 => Ok(Instruction::Fload2),
        x if x == FLOAD_3 => Ok(Instruction::Fload3),
        x if x == FMUL => Ok(Instruction::Fmul),
        x if x == FNEG => Ok(Instruction::Fneg),
        x if x == FREM => Ok(Instruction::Frem),
        x if x == FRETURN => Ok(Instruction::Freturn),
        x if x == FSTORE => {
            let index = read_u8(bytes)?;

            Ok(Instruction::Fstore { index })
        },
        x if x == FSTORE_0 => Ok(Instruction::Fstore0),
        x if x == FSTORE_1 => Ok(Instruction::Fstore1),
        x if x == FSTORE_2 => Ok(Instruction::Fstore2),
        x if x == FSTORE_3 => Ok(Instruction::Fstore3),
        x if x == FSUB => Ok(Instruction::Fsub),
        x if x == GETFIELD => {
            let index = read_u16(bytes)?;

            Ok(Instruction::Getfield { index })
        },
        x if x == GETSTATIC => {
            let index = read_u16(bytes)?;

            Ok(Instruction::Getstatic { index })
        },
        x if x == GOTO => {
            let branchbyte1 = read_u8(bytes)?;
            let branchbyte2 = read_u8(bytes)?;

            Ok(Instruction::Goto { branchbyte1, branchbyte2 })
        },
        x if x == GOTO_W => {
            let indexbyte1 = read_u8(bytes)?;
            let indexbyte2 = read_u8(bytes)?;
            let indexbyte3 = read_u8(bytes)?;
            let indexbyte4 = read_u8(bytes)?;

            Ok(Instruction::GotoW(indexbyte1, indexbyte2, indexbyte3, indexbyte4))
        },
        x if x == I2B => Ok(Instruction::I2b),
        x if x == I2C => Ok(Instruction::I2c),
        x if x == I2D => Ok(Instruction::I2d),
        x if x == I2F => Ok(Instruction::I2f),
        x if x == I2S => Ok(Instruction::I2s),
        x if x == I2L => Ok(Instruction::I2l),
        x if x == IADD => Ok(Instruction::Iadd),
        x if x == IALOAD => Ok(Instruction::Iaload),
        x if x == IAND => Ok(Instruction::Iand),
        x if x == IASTORE => Ok(Instruction::Iastore),
        x if x == ICONST_M1 => Ok(Instruction::IconstM1),
        x if x == ICONST_0 => Ok(Instruction::Iconst0),
        x if x == ICONST_1 => Ok(Instruction::Iconst1),
        x if x == ICONST_2 => Ok(Instruction::Iconst2),
        x if x == ICONST_3 => Ok(Instruction::Iconst3),
        x if x == ICONST_4 => Ok(Instruction::Iconst4),
        x if x == ICONST_5 => Ok(Instruction::Iconst5),
        x if x == IDIV => Ok(Instruction::Idiv),
        x if x == IF_ACMPEQ => {
            let branchbyte1 = read_u8(bytes)?;
            let branchbyte2 = read_u8(bytes)?;

            Ok(Instruction::IfAcmpeq { branchbyte1, branchbyte2 })
        },
        x if x == IF_ACMPNE => {
            let branchbyte1 = read_u8(bytes)?;
            let branchbyte2 = read_u8(bytes)?;

            Ok(Instruction::IfAcmpne { branchbyte1, branchbyte2 })
        },
        x if x == IF_ICMPEQ => {
            let branchbyte1 = read_u8(bytes)?;
            let branchbyte2 = read_u8(bytes)?;

            Ok(Instruction::IfIcmpeq { branchbyte1, branchbyte2 })
        },
        x if x == IF_ICMPNE => {
            let branchbyte1 = read_u8(bytes)?;
            let branchbyte2 = read_u8(bytes)?;

            Ok(Instruction::IfIcmpne { branchbyte1, branchbyte2 })
        },
        x if x == IF_ICMPLT => {
            let branchbyte1 = read_u8(bytes)?;
            let branchbyte2 = read_u8(bytes)?;

            Ok(Instruction::IfIcmplt { branchbyte1, branchbyte2 })
        },
        x if x == IF_ICMPGE => {
            let branchbyte1 = read_u8(bytes)?;
            let branchbyte2 = read_u8(bytes)?;

            Ok(Instruction::IfIcmpge { branchbyte1, branchbyte2 })
        },
        x if x == IF_ICMPGT => {
            let branchbyte1 = read_u8(bytes)?;
            let branchbyte2 = read_u8(bytes)?;

            Ok(Instruction::IfIcmpgt { branchbyte1, branchbyte2 })
        },
        x if x == IF_ICMPLE => {
            let branchbyte1 = read_u8(bytes)?;
            let branchbyte2 = read_u8(bytes)?;

            Ok(Instruction::IfIcmple { branchbyte1, branchbyte2 })
        },
        x if x == IFEQ => {
            let branchbyte1 = read_u8(bytes)?;
            let branchbyte2 = read_u8(bytes)?;

            Ok(Instruction::Ifeq { branchbyte1, branchbyte2 })
        },
        x if x == IFNE => {
            let branchbyte1 = read_u8(bytes)?;
            let branchbyte2 = read_u8(bytes)?;

            Ok(Instruction::Ifne { branchbyte1, branchbyte2 })
        },
        x if x == IFLT => {
            let branchbyte1 = read_u8(bytes)?;
            let branchbyte2 = read_u8(bytes)?;

            Ok(Instruction::Iflt { branchbyte1, branchbyte2 })
        },
        x if x == IFGE => {
            let branchbyte1 = read_u8(bytes)?;
            let branchbyte2 = read_u8(bytes)?;

            Ok(Instruction::Ifge { branchbyte1, branchbyte2 })
        },
        x if x == IFGT => {
            let branchbyte1 = read_u8(bytes)?;
            let branchbyte2 = read_u8(bytes)?;

            Ok(Instruction::Ifgt { branchbyte1, branchbyte2 })
        },
        x if x == IFLE => {
            let branchbyte1 = read_u8(bytes)?;
            let branchbyte2 = read_u8(bytes)?;

            Ok(Instruction::Ifle { branchbyte1, branchbyte2 })
        },
        x if x == IFNONNULL => {
            let branchbyte1 = read_u8(bytes)?;
            let branchbyte2 = read_u8(bytes)?;

            Ok(Instruction::Ifnonnull { branchbyte1, branchbyte2 })
        },
        x if x == IFNULL => {
            let branchbyte1 = read_u8(bytes)?;
            let branchbyte2 = read_u8(bytes)?;

            Ok(Instruction::Ifnull { branchbyte1, branchbyte2 })
        },
        x if x == IINC => {
            let index = read_u8(bytes)?;
            let constant = read_u8(bytes)? as i8;

            Ok(Instruction::Iinc { index, constant })
        },
        x if x == ILOAD => {
            let index = read_u8(bytes)?;

            Ok(Instruction::Iload { index })
        },
        x if x == ILOAD_0 => Ok(Instruction::Iload0),
        x if x == ILOAD_1 => Ok(Instruction::Iload1),
        x if x == ILOAD_2 => Ok(Instruction::Iload2),
        x if x == ILOAD_3 => Ok(Instruction::Iload3),
        x if x == IMUL => Ok(Instruction::Imul),
        x if x == INEG => Ok(Instruction::Ineg),
        x if x == INSTANCEOF => {
            let index = read_u16(bytes)?;

            Ok(Instruction::Instanceof { index })
        },
        x if x == INVOKEDYNAMIC => {
            let index = read_u16(bytes)?;
            let z1 = read_u8(bytes)?;
            let z2 = read_u8(bytes)?;

            Ok(Instruction::Invokedynamic { index})
        },
        x if x == INVOKEINTERFACE => {
            let index = read_u16(bytes)?;
            let count = read_u8(bytes)?;
            let z1 = read_u8(bytes)?;

            Ok(Instruction::Invokeinterface { index, count })
        },
        x if x == INVOKESPECIAL => {
            let index = read_u16(bytes)?;

            Ok(Instruction::Invokespecial { index })
        },
        x if x == INVOKESTATIC => {
            let index = read_u16(bytes)?;

            Ok(Instruction::Invokestatic { index })
        },
        x if x == INVOKEVIRTUAL => {
            let index = read_u16(bytes)?;

            Ok(Instruction::Invokevirtual { index })
        },
        x if x == IOR => Ok(Instruction::Ior),
        x if x == IREM => Ok(Instruction::Irem),
        x if x == IRETURN => Ok(Instruction::Ireturn),
        x if x == ISHL => Ok(Instruction::Ishl),
        x if x == ISHR => Ok(Instruction::Ishr),
        x if x == ISTORE => {
            let index = read_u8(bytes)?;

            Ok(Instruction::Istore { index })
        },
        x if x == ISTORE_0 => Ok(Instruction::Istore0),
        x if x == ISTORE_1 => Ok(Instruction::Istore1),
        x if x == ISTORE_2 => Ok(Instruction::Istore2),
        x if x == ISTORE_3 => Ok(Instruction::Istore3),
        x if x == ISUB => Ok(Instruction::Isub),
        x if x == IUSHR => Ok(Instruction::Iushr),
        x if x == IXOR => Ok(Instruction::Ixor),
        x if x == JSR => {
            let branchbyte1 = read_u8(bytes)?;
            let branchbyte2 = read_u8(bytes)?;

            Ok(Instruction::Jsr { branchbyte1, branchbyte2 })
        },
        x if x == JSR_W => {
            let branchbyte1 = read_u8(bytes)?;
            let branchbyte2 = read_u8(bytes)?;
            let branchbyte3 = read_u8(bytes)?;
            let branchbyte4 = read_u8(bytes)?;

            Ok(Instruction::JsrW { branchbyte1, branchbyte2, branchbyte3, branchbyte4 })
        },
        x if x == L2D => Ok(Instruction::L2d),
        x if x == L2F => Ok(Instruction::L2f),
        x if x == L2I => Ok(Instruction::L2i),
        x if x == LADD => Ok(Instruction::Ladd),
        x if x == LALOAD => Ok(Instruction::Laload),
        x if x == LAND => Ok(Instruction::Land),
        x if x == LASTORE => Ok(Instruction::Lastore),
        x if x == LCMP => Ok(Instruction::Lcmp),
        x if x == LCONST_0 => Ok(Instruction::Lconst0),
        x if x == LCONST_1 => Ok(Instruction::Lconst1),
        x if x == LDC => {
            let index = read_u8(bytes)?;

            Ok(Instruction::Ldc { index })
        },
        x if x == LDC_W => {
            let index = read_u16(bytes)?;

            Ok(Instruction::LdcW { index })
        },
        x if x == LDC2_W => {
            let index = read_u16(bytes)?;

            Ok(Instruction::Ldc2W { index })
        },
        x if x == LDIV => Ok(Instruction::Ldiv),
        x if x == LLOAD => {
            let index = read_u8(bytes)?;

            Ok(Instruction::Lload)
        },
        x if x == LLOAD_0 => Ok(Instruction::Lload0),
        x if x == LLOAD_1 => Ok(Instruction::Lload1),
        x if x == LLOAD_2 => Ok(Instruction::Lload2),
        x if x == LLOAD_3 => Ok(Instruction::Lload3),
        x if x == LMUL => Ok(Instruction::Lmul),
        x if x == LNEG => Ok(Instruction::Lneg),
        x if x == LOOKUPSWITCH => Ok(Instruction::Lookupswitch {}),
        x if x == LOR => Ok(Instruction::Lor),
        x if x == LREM => Ok(Instruction::Lrem),
        x if x == LRETURN => Ok(Instruction::Lreturn),
        x if x == LSHL => Ok(Instruction::Lshl),
        x if x == LSHR => Ok(Instruction::Lshr),
        x if x == LSTORE => {
            let index = read_u8(bytes)?;

            Ok(Instruction::Lstore { index })
        },
        x if x == LSTORE_0 => Ok(Instruction::Lstore0),
        x if x == LSTORE_1 => Ok(Instruction::Lstore1),
        x if x == LSTORE_2 => Ok(Instruction::Lstore2),
        x if x == LSTORE_3 => Ok(Instruction::Lstore3),
        x if x == LSUB => Ok(Instruction::Lsub),
        x if x == LUSHR => Ok(Instruction::Lushr),
        x if x == LXOR => Ok(Instruction::Lxor),
        x if x == MONITORENTER => Ok(Instruction::Monitorenter),
        x if x == MONITOREXIT => Ok(Instruction::Monitorexit),
        x if x == MULTIANEWARRAY => {
            let index = read_u16(bytes)?;
            let dimensions = read_u8(bytes)?;

            Ok(Instruction::Multianewarray { index, dimensions })
        },
        x if x == NEW => {
            let index = read_u16(bytes)?;

            Ok(Instruction::New { index })
        },
        x if x == NEWARRAY => {
            let atype = read_u8(bytes)?;

            Ok(Instruction::Newarray { atype })
        },
        x if x == NOP => Ok(Instruction::Nop),
        x if x == POP => Ok(Instruction::Pop),
        x if x == POP2 => Ok(Instruction::Pop2),
        x if x == PUTFIELD => {
            let index = read_u16(bytes)?;

            Ok(Instruction::Putfield { index })
        },
        x if x == PUTSTATIC => {
            let index = read_u16(bytes)?;

            Ok(Instruction::Putstatic { index })
        },
        x if x == RET => {
            let index = read_u8(bytes)?;

            Ok(Instruction::Ret { index })
        },
        x if x == RETURN => Ok(Instruction::Return),
        x if x == SALOAD => Ok(Instruction::Saload),
        x if x == SASTORE => Ok(Instruction::Sastore),
        x if x == SIPUSH => {
            let byte1 = read_u8(bytes)?;
            let byte2 = read_u8(bytes)?;

            Ok(Instruction::Sipush { byte1, byte2 })
        },
        x if x == SWAP => Ok(Instruction::Swap),
        x if x == TABLESWITCH => Ok(Instruction::Tableswitch {}),
        x if x == WIDE => Ok(Instruction::Wide {}),
        x => Err(DisassemblerError::InvalidOpcode(x))
    }
}

fn read_u8(buffer: &mut Vec<u8>) -> Result<u8, DisassemblerError> {
    match buffer.get(0) {
        Some(&byte) => {
            buffer.remove(0);
            Ok(byte)
        },
        None => Err(DisassemblerError::EndOfCode)
    }
}

fn read_u16(buffer: &mut Vec<u8>) -> Result<u16, DisassemblerError> {
    let b1 = read_u8(buffer)? as u16;
    let b2 = read_u8(buffer)? as u16;

    Ok((b1 << 8) + b2)
}
