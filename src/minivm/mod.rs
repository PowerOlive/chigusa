pub mod codegen;

pub enum Op {
    Nop,
    CPush(u8),
    IPush(u32),
    Pop1,
    Pop2,
    PopN(u32),
    Dup,
    Dup2,
    LoadC(u16),
    LoadA(u16, u32),
    New,
    SNew,
    ILoad,
    DLoad,
    ALoad,
    IALoad,
    DALoad,
    AALoad,
    IStore,
    DStore,
    AStore,
    IAStore,
    DAStore,
    AAStore,
    IAdd,
    DAdd,
    ISub,
    DSub,
    IMul,
    DMul,
    IDiv,
    DDiv,
    INeg,
    DNeg,
    ICmp,
    DCmp,
    I2D,
    D2I,
    I2C,
    Jmp(u16),
    JE(u16),
    JNe(u16),
    JL(u16),
    JGe(u16),
    JG(u16),
    JLe(u16),
    Call(u16),
    Ret,
    IPrint,
    DPrint,
    CPrint,
    SPrint,
    PrintLn,
    IScan,
    DScan,
    CScan,
}

const MAGIC: u32 = (0x43303A29 as u32).to_be();

pub struct FnInfo {}

pub enum Constant {
    Number,
    Float,
    String,
}

pub struct O0 {
    magic: u32,
    version: u32,
    constants: Vec<_>,
    Functions: Vec<_>,
}
