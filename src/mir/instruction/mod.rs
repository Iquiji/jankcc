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
    GetAddrOfLocal(MIRValue, LocalRef),
    /// return value, input value, wanted type
    Deref(MIRValue, MIRValue, MIRType),
    /// store location, input value, provided type
    StoreAtAddr(MIRValue, MIRValue, MIRType),
    IntMath(MIRValue, MIRValue, MIRValue, IntMathKind),
    Compare(MIRValue, MIRValue, MIRValue, IntCmpKind),
    Call(MIRValue, String, Vec<MIRValue>, MIRSignature),
    Return(MIRValue),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum IntMathKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum IntCmpKind {
    Eq,
    UnEq,
    GT,
    LT,
    GET,
    LET,
}
