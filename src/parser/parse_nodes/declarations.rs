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

/*
(6.7.3) type-qualifier:
    const
    restrict
    volatile
    _Atomic
*/

/*
(6.7.4) function-specifier:
    inline
    _Noreturn
*/

/*
(6.7.5) alignment-specifier:
    _Alignas ( type-name )
    _Alignas ( constant-expression )
*/

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

/*
(6.7.8) typedef-name:
    identifier
*/

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

/*
(6.7.10) static_assert-declaration:
    _Static_assert ( constant-expression , string-literal ) ;
*/