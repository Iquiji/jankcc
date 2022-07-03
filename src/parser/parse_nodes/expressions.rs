use super::{Identifier, Constant, StringLiteral, declarations::{TypeName, InitializerList}};

/*
(6.5.1) primary-expression:
    identifier
    constant
    string-literal
    ( expression )
    generic-selection
*/
pub(crate) enum PrimaryExpression{
    Identifier(Identifier),
    Constant(Constant),
    StringLiteral(StringLiteral),
    Expression(Expression),
    GenericSelecetion(GenericSelection),
}

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
pub(crate) struct GenericSelection{
    assignment_expression: Box<AssignmentExpression>,
    generic_assoc_list: GenericAssociationList,
}

pub(crate) enum GenericAssociation{
    TypeName{
        type_name: TypeName,
        assignment_expression: Box<AssignmentExpression>,
    },
    Default(AssignmentExpression),
}

pub(crate) type GenericAssociationList = Vec<GenericAssociation>;

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
pub(crate) enum PostfixExpression{
    PrimaryExpression(PrimaryExpression),
    ArraySubscription{
        on: Box<Self>,
        index: Expression,
    },
    FunctionCall{
        on: Box<Self>,
        args: Option<ArgumentExpressionList>,
    },
    MemberAccess{
        on: Box<Self>,
        member: Identifier,
    },
    DereferencedMemberAccess{
        on: Box<Self>,
        member: Identifier,
    },
    IncrementSelf{
        on: Box<Self>,
    },
    DecrementSelf{
        on: Box<Self>,
    },
    /// Compound literals
    TypeInitializer{
        type_to_init: TypeName,
        initializer_list: InitializerList,
    }
}

/*
(6.5.2) argument-expression-list:
    assignment-expression
    argument-expression-list , assignment-expression
*/
pub(crate) type ArgumentExpressionList = Vec<AssignmentExpression>;

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
pub(crate) enum UnaryExpression{
    PostfixExpression(PostfixExpression),
    PrefixIncrementSelf{
        on: Box<Self>,
    },
    PrefixDecrementSelf{
        on: Box<Self>,
    },
    UnaryArithmetic{
        operator: UnaryOperator,
        on: Box<CastExpression>,
    },
    SizeOf{
        on: Box<Self>,
    },
    SizeOfType{
        type_name: TypeName,
    },
    AlignOfType{
        type_name: TypeName,
    },
}

/*
(6.5.3) unary-operator: one of
    & * + - ~ !
*/
#[allow(clippy::upper_case_acronyms)]
pub(crate) enum UnaryOperator{
    AND,
    POINTER,
    POSITIVE,
    NEGATIVE,
    BITWISEINVERT,
    /// negation operator ! is 0 if the value of its operand compares unequal to 0, 1 if the value of its operand compares equal to 0
    BOOLEANINVERT,
}

/*
(6.5.4) cast-expression:
    unary-expression
    ( type-name ) cast-expression
*/

pub(crate) enum CastExpression{
    UnaryExpression(UnaryExpression),
    Cast{
        type_name: TypeName,
        expresion: Box<Self>,
    },
}

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
pub(crate) enum MultiplicativeExpression{
    CastExpression(CastExpression),
    Expression{
        on: Box<Self>,
        operation: MultiplicativeOperator,
        operand: CastExpression,
    },
}
pub(crate) enum MultiplicativeOperator{
    Mult,
    Div,
    Mod,
}

pub(crate) enum AdditiveExpression{
    MultiplicativeExpression(MultiplicativeExpression),
    Expression{
        on: Box<Self>,
        operation: AdditiveOperator,
        operand: MultiplicativeExpression,
    },
}
pub(crate) enum AdditiveOperator{
    Plus,
    Minus,
}

pub(crate) enum ShiftExpression{
    AdditiveExpression(AdditiveExpression),
    Shift{
        on: Box<Self>,
        operation: ShiftOperator,
        operand: AdditiveExpression,
    },
}
pub(crate) enum ShiftOperator{
    Left,
    Right,
}

pub(crate) enum RelationalExpression{
    ShiftExpression(ShiftExpression),
    Relational{
        on: Box<Self>,
        operation: RelationalOperator,
        operand: ShiftExpression,
    }
}
pub(crate) enum RelationalOperator{
    Lesser,
    Greater,
    LesserEqual,
    GreaterEqual,
}

pub(crate) enum EqualityExpression{
    RelationalExpression(RelationalExpression),
    EqualityCheck{
        on: Box<Self>,
        operation: EqualityOperator,
        operand: RelationalExpression,
    }
}
pub(crate) enum EqualityOperator{
    Equal,
    NotEqual,
}

pub(crate) enum ANDExpression{
    EqualityExpression(EqualityExpression),
    ANDExpression{
        on: Box<Self>,
        operand: EqualityExpression,
    }
}

pub(crate) enum ExclusiveOrExpression{
    ANDExpression(ANDExpression),
    ExclusiveOrExpression{
        on: Box<Self>,
        operand: ANDExpression,
    }
}

pub(crate) enum InclusiveOrExpression{
    ExclusiveOrExpression(ExclusiveOrExpression),
    InclusiveOrExpression{
        on: Box<Self>,
        operand: ExclusiveOrExpression,
    }
}

pub(crate) enum LogicalANDExpression{
    InclusiveOrExpression(InclusiveOrExpression),
    LogicalANDExpression{
        on: Box<Self>,
        operand: InclusiveOrExpression,
    }
}

pub(crate) enum LogicalORExpression{
    LogicalANDExpression(LogicalANDExpression),
    LogicalORExpression{
        on: Box<Self>,
        operand: LogicalANDExpression,
    }
}

pub(crate) enum ConditionalExpression{
    LogicalORExpression(LogicalORExpression),
    Ternary{
        on: LogicalORExpression,
        if_true: Expression,
        operand: Box<ConditionalExpression>,
    }
}

/*
(6.5.16) assignment-expression:
    conditional-expression
    unary-expression assignment-operator assignment-expression
(6.5.16) assignment-operator: one of
    = *= /= %= += -= <<= >>= &= ^= |=
*/
pub(crate) enum AssignmentExpression{
    ConditionalExpression(ConditionalExpression),
    Assignment{
        unary: UnaryExpression,
        operator: AssignmentOperator,
        value: Box<AssignmentExpression>,
    }
}

pub(crate) enum AssignmentOperator{
    Assign,
    AssignMult,
    AssignDiv,
    AssignMod,
    AssignPlus,
    AssignMinus,
    AssignShiftLeft,
    AssignShiftRight,
    AssignAnd,
    AssignXor,
    AssignOr,
}

/*
(6.5.17) expression:
    assignment-expression
    expression , assignment-expression
*/
pub(crate) enum Expression{
    AssignmentExpression(Box<AssignmentExpression>),
    Chain{
        on: Box<Self>,
        expr: Box<AssignmentExpression>,
    },
}

/*
(6.6) constant-expression:
    conditional-expression
*/
pub(crate) struct ConstantExpression{
    internal: ConditionalExpression,
}
