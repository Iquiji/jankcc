/*
Medium Level Intermediate Representation, gonna be emited by walking the AST in the environment_builder.
Usage:
— Emit Cranelift IR in Stage 1 of the compiler
— Used in backend of Compiler and Stage 2

*/

use serde::{Deserialize, Serialize};

/*
IR:
— consists of Blocks taking Args?
— Stuff only has a type denoting its size in bytes
- Control Flow Graph but non-SSA
- Globals? -> Static Vals as Pointer :)
*/
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MIR_Programm {
    pub(crate) globals: Vec<GlobalEntity>,
    pub(crate) constants: Vec<Constant>,
    pub(crate) functions: Vec<MIR_Function>,
}

impl MIR_Programm {
    pub fn new() -> MIR_Programm {
        MIR_Programm {
            globals: vec![],
            constants: vec![],
            functions: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct GlobalEntity {
    pub(crate) name: String,
    pub(crate) extern_linkage: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Constant {
    pub(crate) value: Vec<u8>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MIR_Function {
    pub(crate) name: String,
    pub(crate) blocks: Vec<MIR_Block>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MIR_Block {
    pub(crate) instr: Vec<MIR_Instruction>,
    pub(crate) branches: Option<(MIR_Location, Vec<MIR_Branch>)>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum MIR_Instruction {
    Call(MIR_Location, Vec<MIR_Location>),
    Return(MIR_Location),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum MIR_Location {
    Global(usize, MIR_Type),
    /// Literal Value not Constant like String
    Constant(i64, MIR_Type),
    /// Needs to be Pointer
    ConstantLocation(usize, MIR_Type),
    Local(String, MIR_Type),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MIR_Branch {
    pub(crate) is_default: bool,
    pub(crate) value_needed: u64,
    /// index of block in function
    pub(crate) to_block: usize,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum MIR_Type {
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
}
