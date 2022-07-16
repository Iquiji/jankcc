use serde::{Deserialize, Serialize};

use crate::parser::{span::Spanned, types::{CTypeQualifiers, CTypeSpecifier, CTypeName}};

use super::{expressions::*, Identifier, StringLiteral};


/*
(6.7.10) static_assert-declaration:
    _Static_assert ( constant-expression , string-literal ) ;
*/
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct StaticAssertDeclaration {
    expression: ConstantExpression,
    string_literal: StringLiteral,
}

/*

Struct:
list of declarations,
each declaration:
- specifier-qualifier-list struct-declarator-list?
struct-declarator-list: list of:
- declarator
or
- declarator opt : constant-expression
declarator:
- pointer opt direct-declarator


Struct:
Vec<Decl>
Decl:
- specifier-qualifier-list
 - List of:
  -Normal
   or
  -Initialized

*/
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DeclarationSpecifiers{
    storage: CStorageClass,
    qualifiers: CTypeQualifiers,
    specifiers: CTypeSpecifier,
    function: CFunctionSpecifier,
    alignment: Option<CAlignmentSpecifier>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CStorageClass{
    typedef_c: bool,
    extern_c: bool,
    static_c: bool,
    thread_local_c: bool,
    auto_c: bool,
    register: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CFunctionSpecifier{
    inline: bool,
    no_return: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum CAlignmentSpecifier{
    ToType(CTypeName),
    ToExpression(ConstantExpression),
}

/*
Declarator:

Based or not Based <- Ground Element
Chain of derived types up to None
Based-Type?

*/

/*
Declarator is universal just different versioning for 
type-name
and
declarator
*/
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum DerivedDeclarator{
    Base,
    Pointer{
        qualifiers: CTypeQualifiers,
        to: Box<Self>,
    },
    Binded(Box<Self>),
    Array{
        qualifiers: CTypeQualifiers,
        is_static: bool,
        size_expr: Option<Spanned<CExpression>>,
        // Variable length Array
        vla: bool,
        to: Box<Self>,
    },
    FunctionType{
        parameter_type_list: Spanned<ParameterTypeList>,
        to: Box<Self>,
    },
    FunctionIdentified{
        identifier_list: Vec<Spanned<Identifier>>,
        to: Box<Self>,
    },
}



#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ParameterTypeList{
    parameter_list: Vec<Spanned<ParameterDeclaration>>,
    ellipsis: bool,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ParameterDeclaration{
    // TODO:\
    // declaration-specifiers declarator
    // declaration-specifiers abstract-declarator?
}