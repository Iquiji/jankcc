use crate::{
    lexer::token_types::{CKeyword, CTokenType, CTokenType::*},
    parser::{
        span::{Span, Spanned},
        types::CTypeName,
    },
};

use super::{
    declarations::{InitializerList, TypeName},
    Constant, Identifier, NumberLike, StringLiteral,
};

/*
(6.5.1) primary-expression:
    identifier
    constant
    string-literal
    ( expression )
    generic-selection
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PrimaryExpression {
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GenericSelection {
    assignment_expression: Box<Spanned<CExpression>>,
    generic_assoc_list: Box<Spanned<GenericAssociationList>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum GenericAssociation {
    TypeName {
        type_name: CTypeName,
        assignment_expression: Box<Spanned<CExpression>>,
    },
    Default(Box<Spanned<CExpression>>),
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PostfixExpression {
    PrimaryExpression(PrimaryExpression),
    ArraySubscription {
        on: Box<Self>,
        index: Expression,
    },
    FunctionCall {
        on: Box<Self>,
        args: Option<ArgumentExpressionList>,
    },
    MemberAccess {
        on: Box<Self>,
        member: Identifier,
    },
    DereferencedMemberAccess {
        on: Box<Self>,
        member: Identifier,
    },
    IncrementSelf {
        on: Box<Self>,
    },
    DecrementSelf {
        on: Box<Self>,
    },
    /// Compound literals
    TypeInitializer {
        type_to_init: TypeName,
        initializer_list: InitializerList,
    },
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum UnaryExpression {
    PostfixExpression(PostfixExpression),
    PrefixIncrementSelf {
        on: Box<Self>,
    },
    PrefixDecrementSelf {
        on: Box<Self>,
    },
    UnaryArithmetic {
        operator: UnaryOperator,
        on: Box<CastExpression>,
    },
    SizeOf {
        on: Box<Self>,
    },
    SizeOfType {
        type_name: TypeName,
    },
    AlignOfType {
        type_name: TypeName,
    },
}

/*
(6.5.3) unary-operator: one of
    & * + - ~ !
*/
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum UnaryOperator {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CastExpression {
    UnaryExpression(UnaryExpression),
    Cast {
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MultiplicativeExpression {
    CastExpression(CastExpression),
    Expression {
        on: Box<Self>,
        operation: MultiplicativeOperator,
        operand: CastExpression,
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MultiplicativeOperator {
    Mult,
    Div,
    Mod,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AdditiveExpression {
    MultiplicativeExpression(MultiplicativeExpression),
    Expression {
        on: Box<Self>,
        operation: AdditiveOperator,
        operand: MultiplicativeExpression,
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AdditiveOperator {
    Plus,
    Minus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ShiftExpression {
    AdditiveExpression(AdditiveExpression),
    Shift {
        on: Box<Self>,
        operation: ShiftOperator,
        operand: AdditiveExpression,
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ShiftOperator {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RelationalExpression {
    ShiftExpression(ShiftExpression),
    Relational {
        on: Box<Self>,
        operation: RelationalOperator,
        operand: ShiftExpression,
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RelationalOperator {
    Lesser,
    Greater,
    LesserEqual,
    GreaterEqual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EqualityExpression {
    RelationalExpression(RelationalExpression),
    EqualityCheck {
        on: Box<Self>,
        operation: EqualityOperator,
        operand: RelationalExpression,
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EqualityOperator {
    Equal,
    NotEqual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ANDExpression {
    EqualityExpression(EqualityExpression),
    ANDExpression {
        on: Box<Self>,
        operand: EqualityExpression,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExclusiveOrExpression {
    ANDExpression(ANDExpression),
    ExclusiveOrExpression {
        on: Box<Self>,
        operand: ANDExpression,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum InclusiveOrExpression {
    ExclusiveOrExpression(ExclusiveOrExpression),
    InclusiveOrExpression {
        on: Box<Self>,
        operand: ExclusiveOrExpression,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LogicalANDExpression {
    InclusiveOrExpression(InclusiveOrExpression),
    LogicalANDExpression {
        on: Box<Self>,
        operand: InclusiveOrExpression,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LogicalORExpression {
    LogicalANDExpression(LogicalANDExpression),
    LogicalORExpression {
        on: Box<Self>,
        operand: LogicalANDExpression,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ConditionalExpression {
    LogicalORExpression(LogicalORExpression),
    Ternary {
        on: LogicalORExpression,
        if_true: Expression,
        operand: Box<ConditionalExpression>,
    },
}

/*
(6.5.16) assignment-expression:
    conditional-expression
    unary-expression assignment-operator assignment-expression
(6.5.16) assignment-operator: one of
    = *= /= %= += -= <<= >>= &= ^= |=
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AssignmentExpression {
    ConditionalExpression(ConditionalExpression),
    Assignment {
        unary: UnaryExpression,
        operator: AssignmentOperator,
        value: Box<AssignmentExpression>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AssignmentOperator {
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Expression {
    AssignmentExpression(Box<AssignmentExpression>),
    Chain {
        on: Box<Self>,
        expr: Box<AssignmentExpression>,
    },
}

/*
(6.6) constant-expression:
    conditional-expression
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConstantExpression {
    internal: ConditionalExpression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IncrementType {
    Increment,
    Decrement,
}

/*
expression can be either:

- Expression => Vec<Assignment> seperated by ,
- Assignment => Cond or Assignment ==> difficult
- Conditional <= ConstantExpression (needs to be handled differently because compile time)

arithmetic expression:
- Log Or
- Log And
- inclusive or
- exclusive or
- and
- equality
- relational
- shift
- add
- mult

weird stuff
- cast
- unary
- postfix
- primary

the chain resolves to vec<one_down>
just have to check for operator at the end, if this chaining goes on or not

arithmetic expression?
from log or => mult

assigment,expr,primary,generic seperatily

*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CExpression {
    Assignment {
        to_assign: Box<Spanned<Self>>,
        operator: AssignmentOperator,
        value: Box<Spanned<Self>>,
    },
    Ternary {
        condition: Box<Spanned<Self>>,
        if_true: Box<Spanned<Self>>,
        tern_else: Box<Spanned<Self>>,
    },
    LogicalOr {
        pieces: Vec<Box<Spanned<Self>>>,
    },
    LogicalAnd {
        pieces: Vec<Box<Spanned<Self>>>,
    },
    InclusiveOr {
        pieces: Vec<Box<Spanned<Self>>>,
    },
    ExlusiveOr {
        pieces: Vec<Box<Spanned<Self>>>,
    },
    And {
        pieces: Vec<Box<Spanned<Self>>>,
    },
    Equality {
        left_piece: Box<Spanned<Self>>,
        equality_op: EqualityOperator,
        right_piece: Box<Spanned<Self>>,
    },
    Relational {
        left_piece: Box<Spanned<Self>>,
        equality_op: RelationalOperator,
        right_piece: Box<Spanned<Self>>,
    },
    Shift {
        value: Box<Spanned<Self>>,
        shift_type: ShiftOperator,
        shift_amount: Box<Spanned<Self>>,
    },
    Additive {
        left_value: Box<Spanned<Self>>,
        op: AdditiveOperator,
        right_value: Box<Spanned<Self>>,
    },
    Multiplicative {
        left_value: Box<Spanned<Self>>,
        op: MultiplicativeOperator,
        right_value: Box<Spanned<Self>>,
    },
    Cast {
        type_name: Box<Spanned<CTypeName>>,
        value: Box<Spanned<Self>>,
    },
    PrefixIncrement {
        increment_type: IncrementType,
        value: Box<Spanned<Self>>,
    },
    Unary {
        unary_op: UnaryOperator,
        value: Box<Spanned<Self>>,
    },
    SizeOf {
        value: Box<Spanned<Self>>,
    },
    SizeOfType {
        type_name: Box<Spanned<CTypeName>>,
    },
    AlignOfType {
        type_name: Box<Spanned<CTypeName>>,
    },
    ArraySubscription {
        array: Box<Spanned<Self>>,
        index: Box<Spanned<Self>>,
    },
    FunctionCall {
        function: Box<Spanned<Self>>,
        arguments: Vec<Box<Spanned<Self>>>,
    },
    DirectMemberAccess {
        to_access: Box<Spanned<Self>>,
        member: Identifier,
    },
    IndirectMemberAccess {
        to_access: Box<Spanned<Self>>,
        member: Identifier,
    },
    PostfixIncrement {
        increment_type: IncrementType,
        value: Box<Spanned<Self>>,
    },
    TypeInitializer {
        type_name: CTypeName,
        // FIXME:
        initializer_list: InitializerList,
    },
    Identifier(Identifier),
    Constant(Constant),
    StringLiteral(StringLiteral),
    Paranthesised(Box<Spanned<Self>>),
    GenericSelection(Box<Spanned<GenericSelection>>),
}

impl super::super::CParser {
    pub(crate) fn parse_expression(&mut self) -> Box<Spanned<CExpression>> {
        unimplemented!()
    }
    pub(crate) fn parse_expr_assignment(&mut self) -> Box<Spanned<CExpression>> {
        unimplemented!()
    }
    pub(crate) fn parse_expr_cond(&mut self) -> Box<Spanned<CExpression>> {
        unimplemented!()
    }
    pub(crate) fn parse_expr_logi_or(&mut self) -> Box<Spanned<CExpression>> {
        unimplemented!()
    }
    pub(crate) fn parse_expr_logi_and(&mut self) -> Box<Spanned<CExpression>> {
        unimplemented!()
    }
    pub(crate) fn parse_expr_incl_or(&mut self) -> Box<Spanned<CExpression>> {
        unimplemented!()
    }
    pub(crate) fn parse_expr_excl_or(&mut self) -> Box<Spanned<CExpression>> {
        unimplemented!()
    }
    pub(crate) fn parse_expr_and(&mut self) -> Box<Spanned<CExpression>> {
        unimplemented!()
    }
    pub(crate) fn parse_expr_equality(&mut self) -> Box<Spanned<CExpression>> {
        unimplemented!()
    }
    pub(crate) fn parse_expr_relational(&mut self) -> Box<Spanned<CExpression>> {
        unimplemented!()
    }
    pub(crate) fn parse_expr_shift(&mut self) -> Box<Spanned<CExpression>> {
        unimplemented!()
    }
    pub(crate) fn parse_expr_add(&mut self) -> Box<Spanned<CExpression>> {
        unimplemented!()
    }
    pub(crate) fn parse_expr_mult(&mut self) -> Box<Spanned<CExpression>> {
        unimplemented!()
    }
}
/*
arithmetic expression:
- Log Or
- Log And
- inclusive or
- exclusive or
- and
- equality
- relational
- shift
- add
- mult

weird stuff
- cast
- unary
- postfix
- primary
*/

impl super::super::CParser {
    pub(crate) fn parse_expr_cast(&mut self) -> Box<Spanned<CExpression>> {
        unimplemented!()
    }
    pub(crate) fn parse_expr_unary(&mut self) -> Box<Spanned<CExpression>> {
        unimplemented!()
    }
    pub(crate) fn parse_expr_postfix(&mut self) -> Box<Spanned<CExpression>> {
        /*
            primary-expression
            postfix-expression [ expression ]
            postfix-expression ( argument-expression-listopt )
            postfix-expression . identifier
            postfix-expression -> identifier
            postfix-expression ++
            postfix-expression --
            ( type-name ) { initializer-list }
            ( type-name ) { initializer-list , }

            Idea:
            - Check for ( => then do typeinit ==> check for [ or (
            - parse primary ==> check for [ or (
        */
        let start = self.current_token().loc;
        let mut end = start.clone();

        let initial: Box<Spanned<CExpression>> = if self.current_token().t_type
            == CTokenType::Punctuator
            && self.current_token().original == "("
            && self.check_is_start_of_type_name(&self.next_token())
        {
            // ( type-name ) { initializer-list }
            todo!("type init still unimplemented");
        } else {
            self.parse_expr_primary()
        };

        let mut result = initial;

        // check for '[' or '(' or '.' or '->' or '++' or '--' in a loop
        while self.current_token().t_type == Punctuator {
            let specific_punctuator = self.advance_idx().original;

            match specific_punctuator.as_str() {
                "[" => {
                    unimplemented!()
                }
                "(" => {
                    unimplemented!()
                }
                "." => {
                    let ident = Identifier {
                        string: self.expect_type(Identifier).original,
                    };
                    end = self.current_token().loc;
                    result = Spanned::boxed_new(CExpression::DirectMemberAccess { to_access: result, member: ident }, start.clone(), end.clone());
                }
                "->" => {
                    unimplemented!()
                }
                "++" | "--" => {
                    unimplemented!()
                }
                unknown => panic!("unknown: {}", unknown),
            }
        }

        result
    }
    pub(crate) fn parse_expr_primary(&mut self) -> Box<Spanned<CExpression>> {
        let current_token = self.current_token();
        match current_token.clone().t_type {
            crate::lexer::token_types::CTokenType::Keyword(keyword) => {
                // only GENERIC for generic Selection
                if keyword == CKeyword::GENERIC {
                    // generic selection
                    unimplemented!()
                } else {
                    // panic with unexpected keyword
                    self.error_unexpected(
                        current_token,
                        "Expected only _Generic in primary Expression",
                    );
                    unreachable!()
                }
            }
            crate::lexer::token_types::CTokenType::Identifier => {
                self.advance_idx();
                Spanned::boxed_new(
                    CExpression::Identifier(Identifier {
                        string: current_token.original,
                    }),
                    current_token.loc.clone(),
                    current_token.loc,
                )
            }
            crate::lexer::token_types::CTokenType::Constant => {
                // return constant
                self.advance_idx();
                Spanned::boxed_new(
                    CExpression::Constant(Constant::Number(NumberLike {
                        from: current_token.original,
                    })),
                    current_token.loc.clone(),
                    current_token.loc,
                )
            }
            crate::lexer::token_types::CTokenType::StringLiteral => {
                self.advance_idx();
                Spanned::boxed_new(
                    CExpression::StringLiteral(StringLiteral {
                        value: current_token.original,
                    }),
                    current_token.loc.clone(),
                    current_token.loc,
                )
            }
            crate::lexer::token_types::CTokenType::Punctuator => {
                // only '(' allowed for paranthesised expr
                if current_token.original == "(" {
                    let start = current_token.loc;
                    let expr = self.parse_expression();
                    let end = self.expect_type_and_string(CTokenType::Punctuator, ")").loc;

                    Spanned::boxed_new(CExpression::Paranthesised(expr), start, end)
                } else {
                    // unexpected punctuator -> panic and error?!
                    self.error_unexpected(
                        current_token,
                        "Expected only ( in primary Expression for parenthesised Expression",
                    );
                    unreachable!()
                }
            },
            Eof => unreachable!(),
        }
    }
}
