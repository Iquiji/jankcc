/*
Medium Level Intermediate Representation, gonna be emited by walking the AST in the environment_builder.
Usage:
— Emit Cranelift IR in Stage 1 of the compiler
— Used in backend of Compiler and Stage 2

*/

use bimap::BiBTreeMap;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};

use crate::environment_builder::ext_type::{ExtType, PrettyType};

/*
IR:
— consists of Blocks taking Args?
— Stuff only has a type denoting its size in bytes
- Control Flow Graph but non-SSA
- Globals? -> Static Vals as Pointer :)
*/
pub mod blocks;
mod display_impl;
pub mod function;
pub mod instruction;
pub mod programm;
pub mod types;

pub(crate) use blocks::*;
pub(crate) use function::*;
pub(crate) use instruction::location::*;
pub(crate) use instruction::value::*;
pub(crate) use instruction::*;
pub(crate) use programm::*;
pub(crate) use types::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub(crate) struct MIRConstant {
    pub(crate) value: Vec<u8>,
}

impl MIRConstant {
    pub(crate) fn from_string(data: String) -> Self {
        let mut value = vec![];
        let char_iter = data.chars();

        for character in char_iter {
            // println!("{}",character);
            if character == '\n' {
                value.push(0xA);
            } else {
                value.extend(character.to_string().as_bytes());
            }
        }

        value.push(0);
        MIRConstant { value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MIRSignature {
    pub(crate) return_type: MIRType,
    pub(crate) args: Vec<MIRType>,
    pub(crate) overloadable: bool,
}

impl MIRSignature {
    pub(crate) fn from_function_pretty_type(p_type: &PrettyType) -> Self {
        if let ExtType::Function {
            overextendable,
            returns,
            parameters,
        } = &p_type.inner_type
        {
            Self {
                return_type: MIRType::extract_from_pretty_type(&PrettyType {
                    inner_type: *returns.clone(),
                }),
                args: parameters
                    .iter()
                    .map(|parameter| {
                        MIRType::extract_from_pretty_type(&PrettyType {
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
