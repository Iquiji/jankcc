/*
Medium Level Intermediate Representation, gonna be emited by walking the AST in the environment_builder.
Usage:
— Emit Cranelift IR in Stage 1 of the compiler
— Used in backend of Compiler and Stage 2

*/

use serde::{Deserialize, Serialize};

use crate::environment_builder::ext_type::{ExtType, PrettyType};

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
    pub(crate) signature: MIR_Signature,
    pub(crate) parameter_names: Vec<String>,
    pub(crate) vars: Vec<String>,
    pub(crate) blocks: Vec<MIR_Block>,
    pub(crate) temp_counter: usize,
}
impl MIR_Function{
    pub(crate) fn new() -> MIR_Function{
        MIR_Function {
            name: String::new(),
            blocks: vec![],
            signature: MIR_Signature {
                return_type: MIR_Type::i64,
                args: vec![],
                overloadable: false,
            },
            parameter_names: vec![],
            vars: vec![],
            temp_counter: 0,
        }
    }
    pub(crate) fn make_temp_name(&mut self) -> String{
        let string = format!("_t{}", self.temp_counter);
        self.temp_counter += 1;
        string
    }
    pub(crate) fn make_temp_location(&mut self,mir_type: MIR_Type) -> MIR_Location{
        MIR_Location::Local(self.make_temp_name(), mir_type)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MIR_Signature {
    pub(crate) return_type: MIR_Type,
    pub(crate) args: Vec<MIR_Type>,
    pub(crate) overloadable: bool,
}

impl MIR_Signature {
    pub(crate) fn from_function_pretty_type(p_type: &PrettyType) -> MIR_Signature {
        if let ExtType::Function {
            overextendable,
            returns,
            parameters,
        } = &p_type.inner_type
        {
            MIR_Signature {
                return_type: MIR_Type::extract_from_pretty_type(&PrettyType {
                    inner_type: *returns.clone(),
                }),
                args: parameters
                    .iter()
                    .map(|parameter| {
                        MIR_Type::extract_from_pretty_type(&PrettyType {
                            inner_type: *parameter.parameter_type.clone(),
                        })
                    })
                    .collect(),
                overloadable: *overextendable,
            }
        } else {
            panic!("cannot make MIR function signature out of not function PrettyType")
        }
    }
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
    Add(MIR_Location,MIR_Location,MIR_Location),
    Call(MIR_Location, String, Vec<MIR_Location>,MIR_Signature),
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
impl MIR_Location{
    pub(crate) fn get_mir_type(&self) -> MIR_Type{
        match self{
            MIR_Location::Global(_, mir_type) => mir_type.clone(),
            MIR_Location::Constant(_, mir_type) => mir_type.clone(),
            MIR_Location::ConstantLocation(_, mir_type) => mir_type.clone(),
            MIR_Location::Local(_, mir_type) => mir_type.clone(),
        }
    }
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

impl MIR_Type {
    pub(crate) fn extract_from_pretty_type(p_type: &PrettyType) -> MIR_Type {
        use crate::environment_builder::ext_type::*;
        match &p_type.inner_type {
            ExtType::Void => todo!(),
            ExtType::Int {
                is_const,
                is_volatile,
                signed,
                size,
            } => {
                use MIR_Type::*;
                if *signed {
                    match size {
                        1 => i8,
                        2 => i16,
                        4 => i32,
                        8 => i64,
                        _ => unreachable!(),
                    }
                } else {
                    match size {
                        1 => u8,
                        2 => u16,
                        4 => u32,
                        8 => u64,
                        _ => unreachable!(),
                    }
                }
            }
            ExtType::Float {
                is_const,
                is_volatile,
                size,
            } => todo!(),
            ExtType::Array {
                is_const,
                is_volatile,
                arr_size,
                to,
            } => todo!(),
            ExtType::Pointer {
                is_const,
                is_volatile,
                to,
            } => MIR_Type::i64,
            ExtType::Function {
                overextendable,
                returns,
                parameters,
            } => todo!(),
            ExtType::Struct {
                is_const,
                is_volatile,
                tag,
                alignment,
                members,
            } => todo!(),
            ExtType::Union {
                is_const,
                is_volatile,
                tag,
                members,
            } => todo!(),
        }
    }
}
