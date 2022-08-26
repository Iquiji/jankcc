/*
Qualifiers?

*/

/// A transformed Type from the Parser
pub(crate) enum ExtType{
    Basic{
        is_const: bool,
        is_volatile: bool,
        signed: bool,
        float: bool,
        size: u64,
    },
    Array{
        size: u64,
        of: Box<ExtType>,
        arr_size: u64,
    },
    Pointer{
        is_const: bool,
        is_volatile: bool,
        of: Box<ExtType>,
    },
}

/// Gets size in bytes 
pub(crate) fn get_type_size() -> u64{
    todo!()
}