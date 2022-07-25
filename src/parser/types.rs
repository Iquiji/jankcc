use std::ops::Add;

use log::{trace, info};
use serde::{Deserialize, Serialize};

use crate::lexer::{token_types::{CKeyword, CTokenType}, CToken};

use super::{
    parse_nodes::{
        declarations::{Declarator, DerivedDeclarator, StaticAssertDeclaration},
        expressions::ConstantExpression,
        Identifier,
    },
    span::Spanned,
    CParser,
};
use crate::parser::CTokenType::*;

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
pub(crate) struct CTypeName {
    // need here Basic and then the new derived declarator in declarations.rs

    // this needs to be returned by parse_type_name
    base: Spanned<CTypeBasic>,
    declarator: Spanned<DerivedDeclarator>,
}

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
    struct-or-union-specifier       => struct/union keyword
    enum-specifier                  => enum keyword
    typedef-name                    => check for is_typedef
*/
impl CParser {
    pub(crate) fn parse_type_name(&mut self) -> Spanned<CTypeName> {
        let start = self.current_token().loc;

        Spanned::new(
            CTypeName {
                base: self.parse_specifier_qualifier_list(),
                declarator: Spanned::new(
                    self.parse_abstract_declarator(),
                    start.clone(),
                    self.prev_token().loc,
                ),
            },
            start,
            self.prev_token().loc,
        )
    }

    pub(crate) fn check_is_start_of_type_name(&mut self, token: &CToken) -> bool {
        use super::super::lexer::token_types::CKeyword::*;

        trace!("check_is_start_of_type_name checking: {:?}", token);
        match &token.t_type {
            crate::lexer::token_types::CTokenType::Keyword(keyword) => {
                let possible_type_def_start = [
                    VOID, CHAR, SHORT, INT, LONG, FLOAT, DOUBLE, SIGNED, UNSIGNED, BOOL, COMPLEX,
                    ATOMIC, STRUCT, UNION, ENUM, CONST, RESTRICT, VOLATILE,
                ];
                possible_type_def_start.contains(keyword)
            }
            crate::lexer::token_types::CTokenType::Identifier => self.is_typedef(&token.original),
            crate::lexer::token_types::CTokenType::Constant => false,
            crate::lexer::token_types::CTokenType::StringLiteral => false,
            crate::lexer::token_types::CTokenType::Punctuator => false,
            crate::lexer::token_types::CTokenType::Eof => false,
        }
    }
}
/*
Basic Types:
  - Char
  - Unsigned Int
  - Signed Int
  - Floats
Derived Types make Category:
  - Array
  - Function
  - Pointer
Structured Types:
*/

/*
Types are partitioned into object types (types that describe objects) and function types (types that describe functions).

Basic Types:
  - Char
  - Unsigned Int
  - Signed Int
  - Floats

The basic types are complete object types.
Even if the implementation defines two or more basic types to have the same representation, they are nevertheless different types.

An enumeration comprises a set of named integer constant values. Each distinct
enumeration constitutes a different enumerated type.

The type char, the signed and unsigned integer types, and the enumerated types are
collectively called integer types. The integer and real floating types are collectively called
real types.

Integer and floating types are collectively called arithmetic types. Each arithmetic type
belongs to one type domain: the real type domain comprises the real types, the complex
type domain comprises the complex types.

The void type comprises an empty set of values; it is an incomplete object type that
cannot be completed.

Any number of derived types can be constructed from the object and function types, as
follows:

    — An array type describes a contiguously allocated nonempty set of objects with a
        particular member object type, called the element type. The element type shall be
        complete whenever the array type is specified. Array types are characterized by their
        element type and by the number of elements in the array. An array type is said to be
        derived from its element type, and if its element type is T , the array type is sometimes
        called ‘‘array of T ’’. The construction of an array type from an element type is called
        ‘‘array type derivation’’.
    — A structure type describes a sequentially allocated nonempty set of member objects
        (and, in certain circumstances, an incomplete array), each of which has an optionally
        specified name and possibly distinct type.
    — A union type describes an overlapping nonempty set of member objects, each of
        which has an optionally specified name and possibly distinct type.
    — A function type describes a function with specified return type. A function type is
        characterized by its return type and the number and types of its parameters. A
        function type is said to be derived from its return type, and if its return type is T , the
        function type is sometimes called ‘‘function returning T ’’. The construction of a
        function type from a return type is called ‘‘function type derivation’’.
    — A pointer type may be derived from a function type or an object type, called the
        referenced type. A pointer type describes an object whose value provides a reference
        to an entity of the referenced type. A pointer type derived from the referenced type T
        is sometimes called ‘‘pointer to T ’’. The construction of a pointer type from a
        referenced type is called ‘‘pointer type derivation’’. A pointer type is a complete
        object type.
    — An atomic type describes the type designated by the construct _Atomic ( type-
        name ). (Atomic types are a conditional feature that implementations need not
        support; see 6.10.8.3.)
These methods of constructing derived types can be applied recursively.

Arithmetic types and pointer types are collectively called scalar types.
Array and structure types are collectively called aggregate types.

An array type of unknown size is an incomplete type. It is completed, for an identifier of
that type, by specifying the size in a later declaration (with internal or external linkage).
A structure or union type of unknown content (as described in 6.7.2.3) is an incomplete type.
It is completed, for all declarations of that type, by declaring the same structure or
union tag with its defining content later in the same scope

A type has known constant size if the type is not incomplete and is not a variable length array type.

Array, function, and pointer types are collectively called derived declarator types. A
declarator type derivation from a type T is the construction of a derived declarator type
from T by the application of an array-type, a function-type, or a pointer-type derivation to T .

A type is characterized by its type category, which is either the outermost derivation of a
derived type (as noted above in the construction of derived types), or the type itself if the
type consists of no derived types

Any type so far mentioned is an unqualified type. Each unqualified type has several
qualified versions of its type,47) corresponding to the combinations of one, two, or all
three of the const, volatile, and restrict qualifiers. The qualified or unqualified
versions of a type are distinct types that belong to the same type category and have the
same representation and alignment requirements. A derived type is not qualified by the
qualifiers (if any) of the type from which it is derived

A pointer to void shall have the same representation and alignment requirements as a
pointer to a character type. Similarly, pointers to qualified or unqualified versions of
compatible types shall have the same representation and alignment requirements. All
pointers to structure types shall have the same representation and alignment requirements
as each other. All pointers to union types shall have the same representation and
alignment requirements as each other. Pointers to other types need not have the same
representation or alignment requirements.
*/
/// checks equality disregarding order,
/// checks length
fn basic_ctype_alias_checker(cmp: &[CKeyword]) -> Option<CBasicTypes> {
    use CKeyword::*;

    let mut cmp = cmp.to_vec();
    cmp.dedup_by(|a, b| a == b);
    let cmp = &cmp;

    if is_semi_equal_keywords(&[VOID], cmp) {
        Some(CBasicTypes::Void)
    } else if is_semi_equal_keywords(&[CHAR], cmp) {
        Some(CBasicTypes::Char)
    } else if is_semi_equal_keywords(&[SIGNED, CHAR], cmp) {
        Some(CBasicTypes::SignedChar)
    } else if is_semi_equal_keywords(&[UNSIGNED, CHAR], cmp) {
        Some(CBasicTypes::UnsignedChar)
    } else if is_semi_equal_keywords(&[SHORT], cmp)
        || is_semi_equal_keywords(&[SIGNED, SHORT], cmp)
        || is_semi_equal_keywords(&[SHORT, INT], cmp)
        || is_semi_equal_keywords(&[SIGNED, SHORT, INT], cmp)
    {
        Some(CBasicTypes::Short)
    } else if is_semi_equal_keywords(&[UNSIGNED, SHORT], cmp)
        || is_semi_equal_keywords(&[UNSIGNED, SHORT, INT], cmp)
    {
        Some(CBasicTypes::UnShort)
    } else if is_semi_equal_keywords(&[INT], cmp)
        || is_semi_equal_keywords(&[SIGNED], cmp)
        || is_semi_equal_keywords(&[SIGNED, INT], cmp)
    {
        Some(CBasicTypes::Int)
    } else if is_semi_equal_keywords(&[UNSIGNED], cmp)
        || is_semi_equal_keywords(&[UNSIGNED, INT], cmp)
    {
        Some(CBasicTypes::UnInt)
    } else if is_semi_equal_keywords(&[LONG], cmp)
        || is_semi_equal_keywords(&[SIGNED, LONG], cmp)
        || is_semi_equal_keywords(&[LONG, INT], cmp)
        || is_semi_equal_keywords(&[SIGNED, LONG, INT], cmp)
    {
        Some(CBasicTypes::Long)
    } else if is_semi_equal_keywords(&[UNSIGNED, LONG], cmp)
        || is_semi_equal_keywords(&[UNSIGNED, LONG, INT], cmp)
    {
        Some(CBasicTypes::UnLong)
    } else if is_semi_equal_keywords(&[LONG, LONG], cmp)
        || is_semi_equal_keywords(&[SIGNED, LONG, LONG], cmp)
        || is_semi_equal_keywords(&[LONG, LONG, INT], cmp)
        || is_semi_equal_keywords(&[SIGNED, LONG, LONG, INT], cmp)
    {
        Some(CBasicTypes::LongLong)
    } else if is_semi_equal_keywords(&[UNSIGNED, LONG, LONG], cmp)
        || is_semi_equal_keywords(&[UNSIGNED, LONG, LONG, INT], cmp)
    {
        Some(CBasicTypes::UnLongLong)
    } else if is_semi_equal_keywords(&[FLOAT], cmp) {
        Some(CBasicTypes::Float)
    } else if is_semi_equal_keywords(&[DOUBLE], cmp) {
        Some(CBasicTypes::Double)
    } else if is_semi_equal_keywords(&[LONG, DOUBLE], cmp) {
        Some(CBasicTypes::LongDouble)
    } else if is_semi_equal_keywords(&[BOOL], cmp) {
        Some(CBasicTypes::Bool)
    } else if is_semi_equal_keywords(&[FLOAT, COMPLEX], cmp) {
        Some(CBasicTypes::FloatComplex)
    } else if is_semi_equal_keywords(&[DOUBLE, COMPLEX], cmp) {
        Some(CBasicTypes::DoubleComplex)
    } else if is_semi_equal_keywords(&[LONG, DOUBLE, COMPLEX], cmp) {
        Some(CBasicTypes::LongDoubleComplex)
    } else {
        None
    }
}

fn is_semi_equal_keywords(base: &[CKeyword], cmp: &[CKeyword]) -> bool {
    if base.len() != cmp.len() {
        return false;
    }

    for key in base {
        if !cmp.contains(key) {
            return false;
        }
    }

    true
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CTypeQualifiers {
    pub(crate) const_q: bool,
    pub(crate) restrict_q: bool,
    pub(crate) volatile_q: bool,
    pub(crate) atomic_q: bool,
}

impl Add for CTypeQualifiers {
    type Output = CTypeQualifiers;

    fn add(self, rhs: Self) -> Self::Output {
        CTypeQualifiers {
            const_q: self.const_q || rhs.const_q,
            restrict_q: self.restrict_q || rhs.restrict_q,
            volatile_q: self.volatile_q || rhs.volatile_q,
            atomic_q: self.atomic_q || rhs.atomic_q,
        }
    }
}

impl CParser {
    pub(crate) fn parse_type_qualifiers(&mut self) -> Spanned<CTypeQualifiers> {
        let qualifier_possible = [
            CKeyword::CONST,
            CKeyword::RESTRICT,
            CKeyword::VOLATILE,
            CKeyword::ATOMIC,
        ];
        let qualifier_matcher = |key: &CKeyword, quals: &mut CTypeQualifiers| match key {
            CKeyword::CONST => {
                quals.const_q = true;
            }
            CKeyword::RESTRICT => {
                quals.restrict_q = true;
            }
            CKeyword::VOLATILE => {
                quals.volatile_q = true;
            }
            CKeyword::ATOMIC => {
                quals.atomic_q = true;
            }
            _ => unreachable!(),
        };

        let start = self.current_token().loc;

        let mut qualifiers = CTypeQualifiers {
            const_q: false,
            restrict_q: false,
            volatile_q: false,
            atomic_q: false,
        };

        // get beginning qualifiers
        while let Keyword(keyword) = self.current_token().t_type {
            if qualifier_possible.contains(&keyword) {
                self.advance_idx();
                qualifier_matcher(&keyword, &mut qualifiers);
            } else {
                break;
            }
        }

        let end = self.prev_token().loc;

        Spanned::new(qualifiers, start, end)
    }
}

impl CParser {
    pub(crate) fn parse_specifier_qualifier_list(&mut self) -> Spanned<CTypeBasic> {
        let start = self.current_token().loc;

        let qualifier_possible = [
            CKeyword::CONST,
            CKeyword::RESTRICT,
            CKeyword::VOLATILE,
            CKeyword::ATOMIC,
        ];
        let qualifier_matcher = |key: &CKeyword, quals: &mut CTypeQualifiers| match key {
            CKeyword::CONST => {
                quals.const_q = true;
            }
            CKeyword::RESTRICT => {
                quals.restrict_q = true;
            }
            CKeyword::VOLATILE => {
                quals.volatile_q = true;
            }
            CKeyword::ATOMIC => {
                quals.atomic_q = true;
            }
            _ => unreachable!(),
        };

        let basic_specifiers_possible = [
            CKeyword::VOID,
            CKeyword::CHAR,
            CKeyword::SHORT,
            CKeyword::INT,
            CKeyword::LONG,
            CKeyword::FLOAT,
            CKeyword::DOUBLE,
            CKeyword::SIGNED,
            CKeyword::UNSIGNED,
            CKeyword::BOOL,
            CKeyword::COMPLEX,
        ];

        let mut qualifiers = CTypeQualifiers {
            const_q: false,
            restrict_q: false,
            volatile_q: false,
            atomic_q: false,
        };

        // get beginning qualifiers
        while let Keyword(keyword) = self.current_token().t_type {
            if qualifier_possible.contains(&keyword) {
                self.advance_idx();
                qualifier_matcher(&keyword, &mut qualifiers);
            } else {
                break;
            }
        }

        let specifier: CTypeSpecifier;
        if self.current_token().t_type == Identifier {
            // type_defed mode
            specifier = CTypeSpecifier::Typedefed(super::Identifier {
                identifier: self.advance_idx().original,
            });
        } else if let Keyword(keyword) = self.advance_idx().t_type {
            // one of CTypeSpecifier or qualifer! can be intermixed
            if basic_specifiers_possible.contains(&keyword) {
                // basic type mode
                let mut type_keyword_list: Vec<CKeyword> = vec![keyword];
                // add further:
                while let Keyword(keyword) = self.current_token().t_type {
                    if qualifier_possible.contains(&keyword) {
                        self.advance_idx();
                        qualifier_matcher(&keyword, &mut qualifiers);
                    } else if basic_specifiers_possible.contains(&keyword) {
                        self.advance_idx();
                        type_keyword_list.push(keyword.clone());
                    } else {
                        self.error_unexpected(
                            self.current_token(),
                            "expected basic specifier keyword",
                        );
                        unreachable!()
                    }
                }
                if let Some(basic_specifier) = basic_ctype_alias_checker(&type_keyword_list) {
                    specifier = CTypeSpecifier::Basic(basic_specifier);
                } else {
                    self.error_unexpected(self.prev_token(), "Invalid basic type specifier!");
                    unreachable!()
                }
            } else if keyword == CKeyword::ATOMIC {
                // Atomic Type specifier mode
                self.expect_type_and_string(Punctuator, "(");

                let type_name = self.parse_type_name();

                self.expect_type_and_string(Punctuator, ")");

                specifier = CTypeSpecifier::Atomic(type_name);
                info!("_Atomic is still not properly implemented");
            } else if keyword == CKeyword::ENUM {
                // Enum mode
                // needs to be exported to declaration?
                specifier = CTypeSpecifier::Enum(self.parse_enum_specifier());
                info!("enum specifier is only minimally tested, caution is advised");
            } else if keyword == CKeyword::STRUCT || keyword == CKeyword::UNION {
                // struct or union mode
                self.idx -= 1; // for detection in self.parse_struct_or_union_specifier()
                specifier = CTypeSpecifier::StructOrUnion(self.parse_struct_or_union_specifier());
                // done!("still need to impl struct or union specifier in type name")
            } else {
                // unexpected keyword in specifier qualifier list
                self.error_unexpected(
                    self.prev_token(),
                    "expected valid type name specifier in specifier-qualifier-list",
                );
                unreachable!()
            }
        } else {
            self.error_unexpected(
                self.current_token(),
                "expected type name or type keyword in specifier-qualifier-list",
            );
            unreachable!();
        }

        let end = self.prev_token().loc;

        Spanned::new(
            CTypeBasic {
                qualifiers,
                specifier,
            },
            start,
            end,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CTypeBasic {
    pub(crate) qualifiers: CTypeQualifiers,
    pub(crate) specifier: CTypeSpecifier,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum CBasicTypes {
    Void,
    Char,
    SignedChar,
    UnsignedChar,
    Short,
    UnShort,
    Int,
    UnInt,
    Long,
    UnLong,
    LongLong,
    UnLongLong,
    Float,
    Double,
    LongDouble,
    Bool,
    FloatComplex,
    DoubleComplex,
    LongDoubleComplex,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum CTypeSpecifier {
    Basic(CBasicTypes),
    StructOrUnion(Spanned<CStructOrUnionType>),
    Enum(Spanned<CEnumType>),
    Typedefed(Identifier),
    Atomic(Spanned<CTypeName>)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CStructOrUnionType {
    pub(crate) struct_type: CStructOrUnionTypeType,
    pub(crate) ident: Option<Identifier>,
    pub(crate) declarations: Vec<CSructDeclaration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum CStructOrUnionTypeType {
    Struct,
    Union,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum CSructDeclaration {
    StaticAssertDeclaration(Spanned<StaticAssertDeclaration>),
    StructDeclaration {
        specifier_qualifier: Spanned<CTypeBasic>,
        delcarator_list: Vec<Spanned<CStructDeclarator>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum CStructDeclarator {
    Declarator(Spanned<Declarator>),
    BitField {
        declarator: Option<Spanned<Declarator>>,
        expr: ConstantExpression,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CEnumType {
    pub(crate) ident: Option<Identifier>,
    pub(crate) enumerators: Vec<Spanned<CEnumEnumerator>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CEnumEnumerator {
    pub(crate) enumeration_constant: Identifier,
    pub(crate) const_assignment: Option<ConstantExpression>,
}