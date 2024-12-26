#[allow(dead_code)]
#[derive(Debug)]
pub enum MicroprocessorErrors {
    BpError,
    MemoryOverflow,
    InstructionParamError,
    PushUnsignedWrongMode
}