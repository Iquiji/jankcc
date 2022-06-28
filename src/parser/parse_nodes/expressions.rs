/*
(6.5.1) primary-expression:
    identifier
    constant
    string-literal
    ( expression )
    generic-selection
*/


/*
(6.5.1.1) generic-selection:
    _Generic ( assignment-expression , generic-assoc-list )
(6.5.1.1) generic-assoc-list:
    generic-association
    generic-assoc-list , generic-association
(6.5.1.1) generic-association:
    type-name : assignment-expression
    default : assignment-expression
*/

/*
(6.5.2) postfix-expression:
    primary-expression
    postfix-expression [ expression ]
    postfix-expression ( argument-expression-listopt )
    postfix-expression . identifier
    postfix-expression -> identifier
    postfix-expression ++
    postfix-expression --
    ( type-name ) { initializer-list }
    ( type-name ) { initializer-list , }
*/

/*
(6.5.2) argument-expression-list:
    assignment-expression
    argument-expression-list , assignment-expression
*/

/*
(6.5.3) unary-expression:
    postfix-expression
    ++ unary-expression
    -- unary-expression
    unary-operator cast-expression
    sizeof unary-expression
    sizeof ( type-name )
    _Alignof ( type-name )
*/

/*
(6.5.3) unary-operator: one of
    & * + - ~ !
*/

/*
(6.5.4) cast-expression:
    unary-expression
    ( type-name ) cast-expression
*/

/*
(6.5.5) multiplicative-expression:
    cast-expression
    multiplicative-expression * cast-expression
    multiplicative-expression / cast-expression
    multiplicative-expression % cast-expression
(6.5.6) additive-expression:
    multiplicative-expression
    additive-expression + multiplicative-expression
    additive-expression - multiplicative-expression
(6.5.7) shift-expression:
    additive-expression
    shift-expression << additive-expression
    shift-expression >> additive-expression
(6.5.8) relational-expression:
    shift-expression
    relational-expression < shift-expression
    relational-expression > shift-expression
    relational-expression <= shift-expression
    relational-expression >= shift-expression
(6.5.9) equality-expression:
    relational-expression
    equality-expression == relational-expression
    equality-expression != relational-expression
(6.5.10) AND-expression:
    equality-expression
    AND-expression & equality-expression
(6.5.11) exclusive-OR-expression:
    AND-expression
    exclusive-OR-expression ^ AND-expression
(6.5.12) inclusive-OR-expression:
    exclusive-OR-expression
    inclusive-OR-expression | exclusive-OR-expression
(6.5.13) logical-AND-expression:
    inclusive-OR-expression
    logical-AND-expression && inclusive-OR-expression
(6.5.14) logical-OR-expression:
    logical-AND-expression
    logical-OR-expression || logical-AND-expression
(6.5.15) conditional-expression:
    logical-OR-expression
    logical-OR-expression ? expression : conditional-expression
*/

/*
(6.5.16) assignment-expression:
    conditional-expression
    unary-expression assignment-operator assignment-expression
(6.5.16) assignment-operator: one of
    = *= /= %= += -= <<= >>= &= ^= |=
*/

/*
(6.5.17) expression:
    assignment-expression
    expression , assignment-expression
*/


/*
(6.6) constant-expression:
    conditional-expression
*/