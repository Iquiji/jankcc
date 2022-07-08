use serde::{Deserialize, Serialize};

use crate::lexer::CToken;

use super::{span::Spanned, CParser};

// we need to save the amount of bytes needed to represent
// different sized char,int,unsigned int
// function pointer
// union & struct
// arrays
// type qualifiers: const (C89), volatile (C89), restrict (C99) and _Atomic (C11)
/*
(6.7.7) type-name:
    specifier-qualifier-list abstract-declarator opt
*/
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CTypeName {}

/*
List of either type-specifier or type-qualifier
(6.7.2.1) specifier-qualifier-list:
    type-specifier specifier-qualifier-list opt
    type-qualifier specifier-qualifier-list opt
*/

/*
EZ
(6.7.3) type-qualifier:
    const
    restrict
    volatile
    _Atomic
*/

/*
(6.7.2) type-specifier:
    void
    char
    short
    int
    long
    float
    double
    signed
    unsigned
    _Bool
    _Complex
    atomic-type-specifier           =-=-> _Atomic ( type-name )
    struct-or-union-specifier
    enum-specifier
    typedef-name
*/
impl CParser {
    pub(crate) fn parse_type_name(&mut self) -> Box<Spanned<CTypeName>> {
        unimplemented!()
    }

    pub(crate) fn check_is_start_of_type_name(&mut self, token: &CToken) -> bool {
        unimplemented!()
    }
}
