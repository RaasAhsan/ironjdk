
pub enum Instruction {
    Aaload,
    Aastore,
    AconstNull,
    Aload { index: u8 },
    Aload0,
    Aload1,
    Aload2,
    Aload3,
    Anewarray { indexbyte1: u8, indexbyte2: u8 },
    Areturn,
    Arraylength,
    Astore { index: u8 },
    Astore0,
    Astore1,
    Astore2,
    Astore3,
    Athrow,
    Baload,
    Bastore,
    Bipush { byte: u8 },
    Castore,
    Checkcast { indexbyte1: u8, indexbyte2: u8 },
    D2f,
    D2i,
    D2l,
    Dadd,
    Daload,
    Dastore,
    Dcmpg,
    Dcmpl,
    Dconst0,
    Dconst1,
    Ddiv,
    Dload { index: u8 },
    Dload0,
    Dload1,
    Dload2,
    Dload3,
    Dmul,
    Dneg,
    Drem,
    Dreturn,
    Dstore { index: u8 },
    Dstore0,
    Dstore1,
    Dstore2,
    Dstore3,
    Dsub,
    Dup,
    DupX1,
    DupX2,
    Dup2,
    Dup2X1,
    Dup2X2,
    F2d,
    F2i,
    F2l,
    Fadd,
    Faload,
    Fastore,
    Fcmpg,
    Fcmpl,
    Fconst0,
    Fconst1,
    Fconst2,
    Fdiv,
    Fload { index: u8 },
    Fload0,
    Fload1,
    Fload2,
    Fload3,
    Fmul,
    Fneg,
    Frem,
    Freturn,
    Fstore { index: u8 },
    Fstore0,
    Fstore1,
    Fstore2,
    Fstore3,
    Fsub,
    Getfield { indexbyte1: u8, indexbyte2: u8 },
    Getstatic { indexbyte1: u8, indexbyte2: u8 },
    Goto { branchbyte1: u8, branchbyte2: u8 },
    GotoW { branchbyte1: u8, branchbyte2: u8, branchbyte3: u8, branchbyte4: u8 },
    I2b,
    I2c,
    I2d,
    I2f,
    I2l,
    I2s,
    Iadd,
    Iaload,
    Iand,
    Iastore,
    IconstM1,
    Iconst0,
    Iconst1,
    Iconst2,
    Iconst3,
    Iconst4,
    Iconst5,
    Idiv,
    IfAcmpeq { branchbyte1: u8, branchbyte2: u8 },
    IfAcmpne { branchbyte1: u8, branchbyte2: u8 },
    IfIcmpeq { branchbyte1: u8, branchbyte2: u8 },
    IfIcmpne { branchbyte1: u8, branchbyte2: u8 },
    IfIcmplt { branchbyte1: u8, branchbyte2: u8 },
    IfIcmpge { branchbyte1: u8, branchbyte2: u8 },
    IfIcmpgt { branchbyte1: u8, branchbyte2: u8 },
    IfIcmple { branchbyte1: u8, branchbyte2: u8 },
    Ifeq { branchbyte1: u8, branchbyte2: u8 },
    Ifne { branchbyte1: u8, branchbyte2: u8 },
    Iflt { branchbyte1: u8, branchbyte2: u8 },
    Ifge { branchbyte1: u8, branchbyte2: u8 },
    Ifgt { branchbyte1: u8, branchbyte2: u8 },
    Ifle { branchbyte1: u8, branchbyte2: u8 },
    Ifnonnull { branchbyte1: u8, branchbyte2: u8 },
    Ifnull { branchbyte1: u8, branchbyte2: u8 },
    Iinc { index: u8, constant: u8 },
    Iload { index: u8 },
    Iload0,
    Iload1,
    Iload2,
    Iload3,
    Imul,
    Ineg,
    Instanceof { indexbyte1: u8, indexbyte2: u8 },
    Invokedynamic { indexbyte1: u8, indexbyte2: u8 },
    Invokeinterface { indexbyte1: u8, indexbyte2: u8, count: u8 },
    Invokespecial { indexbyte1: u8, indexbyte2: u8 },
    Invokestatic { indexbyte1: u8, indexbyte2: u8 },
    Invokevirtual { indexbyte1: u8, indexbyte2: u8 },
    Ior,
    Irem,
    Ireturn,
    Ishl,
    Ishr,
    Istore { index: u8 },
    Istore0,
    Istore1,
    Istore2,
    Istore3,
    Isub,
    Iushr,
    Ixor,
    Jsr { branchbyte1: u8, branchbyte2: u8 },
    JsrW { branchbyte1: u8, branchbyte2: u8, branchbyte3: u8, branchbyte4: u8 },
    L2d,
    L2f,
    L2i,
    Ladd,
    Laload,
    Land,
    Lastore,
    Lcmp,
    Lconst0,
    Lconst1,
    Ldc { index: u8 },
    LdcW { indexbyte1: u8, indexbyte2: u8 },
    Ldc2W { indexbyte1: u8, indexbyte2: u8 },
    Ldiv,
    Lload,
    Lload0,
    Lload1,
    Lload2,
    Lload3,
    Lmul,
    Lneg,
    Lookupswitch {},
    Lor,
    Lrem,
    Lreturn,
    Lshl,
    Lshr,
    Lstore { index: u8 },
    Lstore0,
    Lstore1,
    Lstore2,
    Lstore3,
    Lsub,
    Lushr,
    Lxor,
    Monitorenter,
    Monitorexit,
    Multianewarray { indexbyte1: u8, indexbyte2: u8, dimensions: u8 },
    New { indexbyte1: u8, indexbyte2: u8 },
    Newarray { atype: u8 },
    Nop,
    Pop,
    Pop2,
    Putfield { indexbyte1: u8, indexbyte2: u8 },
    Putstatic { indexbyte1: u8, indexbyte2: u8 },
    Ret { index: u8 },
    Return,
    Saload,
    Sastore,
    Sipush { byte1: u8, byte2: u8 },
    Swap,
    Tableswitch {},
    Wide {}
}
