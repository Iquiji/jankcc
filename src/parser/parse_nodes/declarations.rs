use super::{expressions::*, Identifier, StringLiteral};

/*
(6.7) declaration:
    declaration-specifiers init-declarator-list opt ;
    static_assert-declaration
(6.7) declaration-specifiers:
    storage-class-specifier declaration-specifiersopt
    type-specifier declaration-specifiersopt
    type-qualifier declaration-specifiersopt
    function-specifier declaration-specifiersopt
    alignment-specifier declaration-specifiersopt
(6.7) init-declarator-list:
    init-declarator
    init-declarator-list , init-declarator
(6.7) init-declarator:
    declarator
    declarator = initializer
*/
#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum Declaration {
    Normal {
        declaration_specifiers: DeclarationSpecifiers,
        init_declaratior_list: Option<InitDeclaratorList>,
    },
    StaticAssertDeclaration(StaticAssertDeclaration),
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum DeclarationSpecifiers {
    StorageClassSpecifier(StorageClassSpecifier, Option<Box<DeclarationSpecifiers>>),
    TypeSpecifier(TypeSpecifier, Option<Box<DeclarationSpecifiers>>),
    TypeQualifier(TypeQualifier, Option<Box<DeclarationSpecifiers>>),
    FunctionSpecifier(FunctionSpecifier, Option<Box<DeclarationSpecifiers>>),
    AlignmentSpecifier(AlignmentSpecifier, Option<Box<DeclarationSpecifiers>>),
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum InitDeclarator {
    Normal(Declarator),
    Initialized(Declarator, Initializer),
}
pub(crate) type InitDeclaratorList = Vec<InitDeclarator>;

/*
(6.7.1) storage-class-specifier:
    typedef
    extern
    static
    _Thread_local
    auto
    register
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
    atomic-type-specifier
    struct-or-union-specifier
    enum-specifier
    typedef-name
(6.7.2.1) struct-or-union-specifier:
    struct-or-union identifieropt { struct-declaration-list }
    struct-or-union identifier
(6.7.2.1) struct-or-union:
    struct
    union
(6.7.2.1) struct-declaration-list:
    struct-declaration
    struct-declaration-list struct-declaration
(6.7.2.1) struct-declaration:
    specifier-qualifier-list struct-declarator-listopt ;
    static_assert-declaration
(6.7.2.1) specifier-qualifier-list:
    type-specifier specifier-qualifier-listopt
    type-qualifier specifier-qualifier-listopt
(6.7.2.1) struct-declarator-list:
    struct-declarator
    struct-declarator-list , struct-declarator
(6.7.2.1) struct-declarator:
    declarator
    declarator opt : constant-expression
(6.7.2.2) enum-specifier:
    enum identifier opt { enumerator-list }
    enum identifier opt { enumerator-list , }
    enum identifier
(6.7.2.2) enumerator-list:
    enumerator
    enumerator-list , enumerator
(6.7.2.2) enumerator:
    enumeration-constant
    enumeration-constant = constant-expression
(6.7.2.4) atomic-type-specifier:
    _Atomic ( type-name )
*/
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum StorageClassSpecifier {
    TYPEDEF,
    EXTERN,
    STATIC,
    THREADLOCAL,
    AUTO,
    REGISTER,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum TypeSpecifier {
    VOID,
    CHAR,
    SHORT,
    INT,
    LONG,
    FLOAT,
    DOUBLE,
    SIGNED,
    UNSIGNED,
    BOOL,
    COMPLEX,
    Atomic(TypeName),
    StructOrUnion(StructOrUnionSpecifier),
    Enum(EnumSpecifier),
    TypedefName(TypedefName),
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum StructOrUnionSpecifier {
    Declare {
        identifier: Option<Identifier>,
        declaration_list: StructDeclarationList,
    },
    Identifier(Identifier),
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum StructOrUnion {
    Struct,
    Union,
}

pub(crate) type StructDeclarationList = Vec<StructDeclaration>;

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum StructDeclaration {
    Normal {
        specifier_qualifier_list: SpecifierQualifierList,
        declarator_list: Option<StructDeclaratorList>,
    },
    StaticAssert(StaticAssertDeclaration),
}

pub(crate) type StructDeclaratorList = Vec<StructDeclarator>;

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum SpecifierQualifier {
    TypeSpecifier(TypeSpecifier),
    TypeQualifier(TypeQualifier),
}

pub(crate) type SpecifierQualifierList = Vec<SpecifierQualifier>;

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum StructDeclarator {
    Normal(Declarator),
    ConstExpr {
        declarator: Option<Declarator>,
        constant_expression: ConstantExpression,
    },
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum EnumSpecifier {
    List {
        identifier: Option<Identifier>,
        enumerator_list: EnumeratorList,
    },
    Identifier(Identifier),
}

pub(crate) type EnumeratorList = Vec<Enumerator>;

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum Enumerator {
    Const(EnumerationConstant),
    ConstEquals {
        enumeration_constant: EnumerationConstant,
        constant_expression: ConstantExpression,
    },
}

pub(crate) type EnumerationConstant = Identifier;

/*
(6.7.3) type-qualifier:
    const
    restrict
    volatile
    _Atomic
*/
#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum TypeQualifier {
    Const,
    Restrict,
    Volatile,
    Atomic,
}

/*
(6.7.4) function-specifier:
    inline
    _Noreturn
*/
#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum FunctionSpecifier {
    Inline,
    Noreturn,
}

/*
(6.7.5) alignment-specifier:
    _Alignas ( type-name )
    _Alignas ( constant-expression )
*/
#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum AlignmentSpecifier {
    AsType(TypeName),
    AsConstExpr(ConstantExpression),
}

/*
(6.7.6) declarator:
    pointer opt direct-declarato
(6.7.6) direct-declarator:
    identifier
    ( declarator )
    direct-declarator [ type-qualifier-list opt assignment-expressionopt ]
    direct-declarator [ static type-qualifier-listopt assignment-expression ]
    direct-declarator [ type-qualifier-list static assignment-expression ]
    direct-declarator [ type-qualifier-list opt * ]
    direct-declarator ( parameter-type-list )
    direct-declarator ( identifier-listopt )
(6.7.6) pointer:
    * type-qualifier-listopt
    * type-qualifier-list opt pointer
(6.7.6) type-qualifier-list:
    type-qualifier
    type-qualifier-list type-qualifier
(6.7.6) parameter-type-list:
    parameter-list
    parameter-list , ...
(6.7.6) parameter-list:
    parameter-declaration
    parameter-list , parameter-declaration
(6.7.6) parameter-declaration:
    declaration-specifiers declarator
    declaration-specifiers abstract-declaratoropt
(6.7.6) identifier-list:
    identifier
    identifier-list , identifier
*/
#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) struct Declarator {
    pointer: Option<Pointer>,
    direct: Box<DirectDeclarator>,
}
#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum DirectDeclarator {
    Identifier(Identifier),
    Declarator(Declarator),
    IndexedNormal {
        qualifier: Option<TypeQualifierList>,
        assignment: Option<AssignmentExpression>,
    },
    IndexedStatic {
        qualifier: Option<TypeQualifierList>,
        assignment: AssignmentExpression,
    },
    IndexedStaticType2 {
        qualifier: TypeQualifierList,
        assignment: AssignmentExpression,
    },
    IndexedStar {
        qualifier: Option<TypeQualifierList>,
    },
    Called(ParameterTypeList),
    CalledType2(Option<IdentifierList>),
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) struct Pointer {
    qualifier_list: TypeQualifierList,
    opt_next: Box<Pointer>,
}

pub(crate) type TypeQualifierList = Vec<TypeQualifier>;

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) struct ParameterTypeList {
    parameter_list: ParameterList,
    /// flag for has elipsis
    ellipsis: bool,
}

pub(crate) type ParameterList = Vec<ParameterDeclaration>;
pub(crate) type IdentifierList = Vec<Identifier>;

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum ParameterDeclaration {
    Normal {
        specifier: DeclarationSpecifiers,
        declarator: Declarator,
    },
    Abstract {
        specifier: DeclarationSpecifiers,
        abstract_declarator: AbstractDeclarator,
    },
}

/*
(6.7.7) type-name:
    specifier-qualifier-list abstract-declarator opt
(6.7.7) abstract-declarator:
    pointer
    pointer opt direct-abstract-declarator
(6.7.7) direct-abstract-declarator:
    ( abstract-declarator )
    direct-abstract-declarator opt [ type-qualifier-list opt
    assignment-expressionopt ]
    direct-abstract-declaratoropt [ static type-qualifier-listopt
    assignment-expression ]
    direct-abstract-declaratoropt [ type-qualifier-list static
    assignment-expression ]
    direct-abstract-declaratoropt [ * ]
    direct-abstract-declaratoropt ( parameter-type-list opt )
*/
#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) struct TypeName {
    specifiers: SpecifierQualifierList,
    abstract_declarator: Option<Box<AbstractDeclarator>>,
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum AbstractDeclarator {
    Pointer(Pointer),
    Direct {
        pointer: Option<Pointer>,
        direct: DirectAbstractDeclarator,
    },
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum DirectAbstractDeclarator {
    Abstract(Box<AbstractDeclarator>),
    IndexedNormal {
        chain: Option<Box<DirectAbstractDeclarator>>,
        qualifier: Option<TypeQualifierList>,
        assignment: Option<AssignmentExpression>,
    },
    IndexedStatic {
        chain: Option<Box<DirectAbstractDeclarator>>,
        qualifier: Option<TypeQualifierList>,
        assignment: AssignmentExpression,
    },
    IndexedStaticType2 {
        chain: Option<Box<DirectAbstractDeclarator>>,
        qualifier: TypeQualifierList,
        assignment: AssignmentExpression,
    },
    IndexedStar {
        chain: Option<Box<DirectAbstractDeclarator>>,
        qualifier: Option<TypeQualifierList>,
    },
    Called {
        chain: Option<Box<DirectAbstractDeclarator>>,
        parameter_type_list: ParameterTypeList,
    },
}

/*
(6.7.8) typedef-name:
    identifier
*/
pub(crate) type TypedefName = Identifier;

/*
(6.7.9) initializer:
    assignment-expression
    { initializer-list }
    { initializer-list , }
(6.7.9) initializer-list:
    designationopt initializer
    initializer-list , designation opt initializer
(6.7.9) designation:
    designator-list =
(6.7.9) designator-list:
    designator
    designator-list designator
(6.7.9) designator:
    [ constant-expression ]
    . identifier
*/
#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum Initializer {
    AssignmentExpression(AssignmentExpression),
    InitializerList(InitializerList),
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) struct InitializerListItem {
    designation: Option<Designation>,
    initializer: Initializer,
}
pub(crate) type InitializerList = Vec<InitializerListItem>;

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) struct Designation {
    designator_list: DesignatorList,
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum Designator {
    Indexed(ConstantExpression),
    Pointed(Identifier),
}

pub(crate) type DesignatorList = Vec<Designator>;

/*
(6.7.10) static_assert-declaration:
    _Static_assert ( constant-expression , string-literal ) ;
*/
#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) struct StaticAssertDeclaration {
    expression: ConstantExpression,
    string_literal: StringLiteral,
}
