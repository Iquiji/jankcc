use super::*;

pub mod location;
pub mod value;

use location::*;
use value::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum MIRInstruction {
    GetConstDataPtr(MIRValue, DataConstantRef),
    ConstNum(MIRValue, i64, MIRType),
    ReadLocal(MIRValue, LocalRef),
    AssignLocal(LocalRef, MIRValue),
    GetAddrOfLocal(MIRValue,LocalRef),
    IntMath(MIRValue, MIRValue, MIRValue,IntMathKind),
    Compare(MIRValue,MIRValue,MIRValue),
    Call(MIRValue, String, Vec<MIRValue>, MIRSignature),
    Return(MIRValue),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum IntMathKind{
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}
