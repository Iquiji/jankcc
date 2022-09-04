use crate::{
    lexer::token_types::{CKeyword, CTokenType, CTokenType::*},
    parser::{span::Spanned, types::CTypeName, CParser},
};

use super::{declarations::Initializer, Constant, Identifier, NumberLike, StringLiteral};

use log::info;
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct GenericSelection {
    assignment_expression: Spanned<CExpression>,
    generic_assoc_list: Box<Spanned<GenericAssociationList>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum GenericAssociation {
    TypeName {
        type_name: CTypeName,
        assignment_expression: Spanned<CExpression>,
    },
    Default(Spanned<CExpression>),
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

/*
(6.5.2) argument-expression-list:
    assignment-expression
    argument-expression-list , assignment-expression
*/
pub(crate) type ArgumentExpressionList = Vec<Spanned<CExpression>>;
impl CParser {
    fn parse_argument_expression_list(&mut self) -> ArgumentExpressionList {
        // println!(
        //     "current: {:?},next: {:?}",
        //     self.current_token(),
        //     self.next_token()
        // );

        let mut args = vec![];

        if self.current_token().t_type == Punctuator && self.current_token().original == ")" {
            return args;
        }

        args.push(self.parse_expr_assignment());

        while self.current_token().t_type == Punctuator && self.current_token().original == "," {
            self.advance_idx();

            if self.current_token().t_type == Punctuator && self.current_token().original == ")" {
                return args;
            }

            args.push(self.parse_expr_assignment());
        }

        args
    }
}

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
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum UnaryOperator {
    REF,
    DEREF,
    VALUE,
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum MultiplicativeOperator {
    Mult,
    Div,
    Mod,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum AdditiveOperator {
    Plus,
    Minus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ShiftOperator {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum RelationalOperator {
    Lesser,
    Greater,
    LesserEqual,
    GreaterEqual,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EqualityOperator {
    Equal,
    NotEqual,
}

/*
(6.5.16) assignment-expression:
    conditional-expression
    unary-expression assignment-operator assignment-expression
(6.5.16) assignment-operator: one of
    = *= /= %= += -= <<= >>= &= ^= |=
*/
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

/*
(6.6) constant-expression:
    conditional-expression
*/
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ConstantExpression {
    pub(crate) internal: Spanned<CExpression>,
}
impl CParser {
    pub(crate) fn parse_constant_expr(&mut self) -> ConstantExpression {
        info!("constant expr unstable");
        ConstantExpression {
            internal: self.parse_expr_cond(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum CExpression {
    Expression(Vec<Spanned<Self>>),
    Assignment {
        to_assign: Spanned<Self>,
        operator: AssignmentOperator,
        value: Spanned<Self>,
    },
    Ternary {
        condition: Spanned<Self>,
        if_true: Spanned<Self>,
        tern_else: Spanned<Self>,
    },
    LogicalOr(Vec<Spanned<Self>>),
    LogicalAnd(Vec<Spanned<Self>>),
    InclusiveOr(Vec<Spanned<Self>>),
    ExlusiveOr(Vec<Spanned<Self>>),
    And(Vec<Spanned<Self>>),
    Equality {
        left_piece: Spanned<Self>,
        equality_op: EqualityOperator,
        right_piece: Spanned<Self>,
    },
    Relational {
        left_piece: Spanned<Self>,
        equality_op: RelationalOperator,
        right_piece: Spanned<Self>,
    },
    Shift {
        value: Spanned<Self>,
        shift_type: ShiftOperator,
        shift_amount: Spanned<Self>,
    },
    Additive {
        left_value: Spanned<Self>,
        op: AdditiveOperator,
        right_value: Spanned<Self>,
    },
    Multiplicative {
        left_value: Spanned<Self>,
        op: MultiplicativeOperator,
        right_value: Spanned<Self>,
    },
    Cast {
        type_name: Spanned<CTypeName>,
        value: Spanned<Self>,
    },
    PrefixIncrement {
        increment_type: IncrementType,
        value: Spanned<Self>,
    },
    Unary {
        unary_op: UnaryOperator,
        value: Spanned<Self>,
    },
    SizeOf {
        value: Spanned<Self>,
    },
    SizeOfType {
        type_name: Spanned<CTypeName>,
    },
    AlignOfType {
        type_name: Spanned<CTypeName>,
    },
    ArraySubscription {
        array: Spanned<Self>,
        index: Spanned<Self>,
    },
    FunctionCall {
        function: Spanned<Self>,
        arguments: Vec<Spanned<Self>>,
    },
    DirectMemberAccess {
        to_access: Spanned<Self>,
        member: Identifier,
    },
    IndirectMemberAccess {
        to_access: Spanned<Self>,
        member: Identifier,
    },
    PostfixIncrement {
        increment_type: IncrementType,
        value: Spanned<Self>,
    },
    TypeInitializer {
        type_name: Spanned<CTypeName>,
        initializer_list: Spanned<Initializer>,
    },
    Identifier(Identifier),
    Constant(Constant),
    StringLiteral(StringLiteral),
    Paranthesised(Spanned<Self>),
    GenericSelection(Box<Spanned<GenericSelection>>),
}

impl super::super::CParser {
    pub(crate) fn parse_expression(&mut self) -> Spanned<CExpression> {
        let start = self.current_token().loc;

        let mut result = self.parse_expr_assignment();

        if self.current_token().original == "," {
            let mut result_vec = vec![result];

            while self.current_token().original == "," {
                self.advance_idx();
                result_vec.push(self.parse_expr_assignment());
            }

            result = Spanned::new(
                CExpression::Expression(result_vec),
                start,
                self.prev_token().loc,
            );
        }

        result
    }
    pub(crate) fn parse_expr_assignment(&mut self) -> Spanned<CExpression> {
        /*
        assignment-expression:
            conditional-expression
            unary-expression assignment-operator assignment-expression

        assignment-operator: one of
            = *= /= %= += -= <<= >>= &= ^= |=
        */
        /*
        Idea:
            Parse Cond Expression;
            then if is of type unary-expr or lower:
                check for assignment operator
            else:
                return conditional_expression
        */
        let possible_extensions = [
            "=", "*=", "/=", "%=", "+=", "-=", "<<=", ">>=", "&=", "^=", "|=",
        ];
        let op_matcher = |op: &str| match op {
            "=" => AssignmentOperator::Assign,
            "*=" => AssignmentOperator::AssignMult,
            "/=" => AssignmentOperator::AssignDiv,
            "%=" => AssignmentOperator::AssignMod,
            "+=" => AssignmentOperator::AssignPlus,
            "-=" => AssignmentOperator::AssignMinus,
            "<<=" => AssignmentOperator::AssignShiftLeft,
            ">>=" => AssignmentOperator::AssignShiftRight,
            "&=" => AssignmentOperator::AssignAnd,
            "^=" => AssignmentOperator::AssignXor,
            "|=" => AssignmentOperator::AssignOr,
            _ => unreachable!(),
        };

        let start = self.current_token().loc;
        let mut result = self.parse_expr_cond();

        use CExpression::*;

        if matches!(
            &*result,
            Unary {
                unary_op: _,
                value: _,
            } | SizeOf { value: _ }
                | SizeOfType { type_name: _ }
                | AlignOfType { type_name: _ }
                | ArraySubscription { array: _, index: _ }
                | FunctionCall {
                    function: _,
                    arguments: _,
                }
                | DirectMemberAccess {
                    to_access: _,
                    member: _,
                }
                | IndirectMemberAccess {
                    to_access: _,
                    member: _,
                }
                | PostfixIncrement {
                    increment_type: _,
                    value: _,
                }
                | TypeInitializer {
                    type_name: _,
                    initializer_list: _,
                }
                | Identifier(_)
                | Constant(_)
                | StringLiteral(_)
                | Paranthesised(_)
                | GenericSelection(_)
        ) && possible_extensions.contains(&self.current_token().original.as_str())
        {
            result = Spanned::new(
                CExpression::Assignment {
                    to_assign: result,
                    operator: op_matcher(&self.advance_idx().original),
                    value: self.parse_expr_assignment(),
                },
                start,
                self.prev_token().loc,
            );
        }

        result
    }
    pub(crate) fn parse_expr_cond(&mut self) -> Spanned<CExpression> {
        /*
        conditional-expression:
            logical-OR-expression
            logical-OR-expression ? expression : conditional-expression
        */
        let start = self.current_token().loc;
        let mut result = self.parse_expr_logi_or();

        if self.current_token().t_type == Punctuator && self.current_token().original == "?" {
            self.advance_idx();
            let if_true = self.parse_expression();
            self.expect_type_and_string(Punctuator, ":");
            let tern_else = self.parse_expr_cond();
            let end = self.prev_token().loc;
            result = Spanned::new(
                CExpression::Ternary {
                    condition: result,
                    if_true,
                    tern_else,
                },
                start,
                end,
            );
        }

        result
    }
    pub(crate) fn parse_expr_logi_or(&mut self) -> Spanned<CExpression> {
        let start = self.current_token().loc;

        let mut result = self.parse_expr_logi_and();

        if self.current_token().original == "||" {
            let mut result_vec = vec![result];

            while self.current_token().original == "||" {
                self.advance_idx();
                result_vec.push(self.parse_expr_logi_and());
            }

            result = Spanned::new(
                CExpression::LogicalAnd(result_vec),
                start,
                self.prev_token().loc,
            );
        }

        result
    }
    pub(crate) fn parse_expr_logi_and(&mut self) -> Spanned<CExpression> {
        let start = self.current_token().loc;

        let mut result = self.parse_expr_incl_or();

        if self.current_token().original == "&&" {
            let mut result_vec = vec![result];

            while self.current_token().original == "&&" {
                self.advance_idx();
                result_vec.push(self.parse_expr_incl_or());
            }

            result = Spanned::new(
                CExpression::LogicalAnd(result_vec),
                start,
                self.prev_token().loc,
            );
        }

        result
    }
    pub(crate) fn parse_expr_incl_or(&mut self) -> Spanned<CExpression> {
        let start = self.current_token().loc;

        let mut result = self.parse_expr_excl_or();

        if self.current_token().original == "|" {
            let mut result_vec = vec![result];

            while self.current_token().original == "|" {
                self.advance_idx();
                result_vec.push(self.parse_expr_excl_or());
            }

            result = Spanned::new(
                CExpression::InclusiveOr(result_vec),
                start,
                self.prev_token().loc,
            );
        }

        result
    }
    pub(crate) fn parse_expr_excl_or(&mut self) -> Spanned<CExpression> {
        let start = self.current_token().loc;

        let mut result = self.parse_expr_and();

        if self.current_token().original == "^" {
            let mut result_vec = vec![result];

            while self.current_token().original == "^" {
                self.advance_idx();
                result_vec.push(self.parse_expr_and());
            }

            result = Spanned::new(
                CExpression::ExlusiveOr(result_vec),
                start,
                self.prev_token().loc,
            );
        }

        result
    }
    pub(crate) fn parse_expr_and(&mut self) -> Spanned<CExpression> {
        let start = self.current_token().loc;

        let mut result = self.parse_expr_equality();

        if self.current_token().original == "&" {
            let mut result_vec = vec![result];

            while self.current_token().original == "&" {
                self.advance_idx();
                result_vec.push(self.parse_expr_equality());
            }

            result = Spanned::new(CExpression::And(result_vec), start, self.prev_token().loc);
        }

        result
    }
    pub(crate) fn parse_expr_equality(&mut self) -> Spanned<CExpression> {
        let possible_extensions = ["==", "!="];
        let op_matcher = |op: &str| match op {
            "==" => EqualityOperator::Equal,
            "!=" => EqualityOperator::NotEqual,
            _ => unreachable!(),
        };

        let start = self.current_token().loc;

        let mut result = self.parse_expr_relational();

        while possible_extensions.contains(&self.current_token().original.as_str()) {
            result = Spanned::new(
                CExpression::Equality {
                    left_piece: result,
                    equality_op: op_matcher(&self.advance_idx().original),
                    right_piece: self.parse_expr_relational(),
                },
                start.clone(),
                self.prev_token().loc,
            )
        }

        result
    }
    pub(crate) fn parse_expr_relational(&mut self) -> Spanned<CExpression> {
        let possible_extensions = ["<", ">", "<=", ">="];
        let op_matcher = |op: &str| match op {
            "<" => RelationalOperator::Lesser,
            ">" => RelationalOperator::Greater,
            "<=" => RelationalOperator::LesserEqual,
            ">=" => RelationalOperator::GreaterEqual,
            _ => unreachable!(),
        };

        let start = self.current_token().loc;

        let mut result = self.parse_expr_shift();

        while possible_extensions.contains(&self.current_token().original.as_str()) {
            result = Spanned::new(
                CExpression::Relational {
                    left_piece: result,
                    equality_op: op_matcher(&self.advance_idx().original),
                    right_piece: self.parse_expr_shift(),
                },
                start.clone(),
                self.prev_token().loc,
            )
        }

        result
    }
    pub(crate) fn parse_expr_shift(&mut self) -> Spanned<CExpression> {
        let possible_extensions = ["<<", ">>"];
        let op_matcher = |op: &str| match op {
            "<<" => ShiftOperator::Left,
            ">>" => ShiftOperator::Right,
            _ => unreachable!(),
        };

        let start = self.current_token().loc;

        let mut result = self.parse_expr_add();

        while possible_extensions.contains(&self.current_token().original.as_str()) {
            result = Spanned::new(
                CExpression::Shift {
                    value: result,
                    shift_type: op_matcher(&self.advance_idx().original),
                    shift_amount: self.parse_expr_add(),
                },
                start.clone(),
                self.prev_token().loc,
            )
        }

        result
    }
    pub(crate) fn parse_expr_add(&mut self) -> Spanned<CExpression> {
        let possible_extensions = ["+", "-"];
        let op_matcher = |op: &str| match op {
            "+" => AdditiveOperator::Plus,
            "-" => AdditiveOperator::Minus,
            _ => unreachable!(),
        };

        let start = self.current_token().loc;

        let mut result = self.parse_expr_mult();

        while possible_extensions.contains(&self.current_token().original.as_str()) {
            result = Spanned::new(
                CExpression::Additive {
                    left_value: result,
                    op: op_matcher(&self.advance_idx().original),
                    right_value: self.parse_expr_mult(),
                },
                start.clone(),
                self.prev_token().loc,
            )
        }

        result
    }
    pub(crate) fn parse_expr_mult(&mut self) -> Spanned<CExpression> {
        /*
        (6.5.5) multiplicative-expression:
            cast-expression
            multiplicative-expression * cast-expression
            multiplicative-expression / cast-expression
            multiplicative-expression % cast-expression
        */
        let possible_extensions = ["*", "/", "%"];
        let op_matcher = |op: &str| match op {
            "*" => MultiplicativeOperator::Mult,
            "/" => MultiplicativeOperator::Div,
            "%" => MultiplicativeOperator::Mod,
            _ => unreachable!(),
        };

        let start = self.current_token().loc;

        let mut result = self.parse_expr_cast();

        while possible_extensions.contains(&self.current_token().original.as_str()) {
            result = Spanned::new(
                CExpression::Multiplicative {
                    left_value: result,
                    op: op_matcher(&self.advance_idx().original),
                    right_value: self.parse_expr_cast(),
                },
                start.clone(),
                self.prev_token().loc,
            )
        }

        result
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
    pub(crate) fn parse_expr_cast(&mut self) -> Spanned<CExpression> {
        /*
        (6.5.4) cast-expression:
            unary-expression
            ( type-name ) cast-expression
        */
        // we need to check for type name here as well
        if self.current_token().t_type == CTokenType::Punctuator
            && self.current_token().original == "("
            && self.check_is_start_of_type_name(&self.next_token())
        {
            // ( type-name ) cast-expression

            // differ between this and init of type name?
            // parse type name and then check for '{'
            let start = self.current_token().loc;
            let idx_before = self.idx;
            self.advance_idx();

            let type_name = self.parse_type_name();

            self.expect_type_and_string(CTokenType::Punctuator, ")");

            if self.current_token().t_type == CTokenType::Punctuator
                && self.current_token().original == "{"
            {
                // initializer
                info!("compound literal in 'type cast' may be buggy");
                self.idx = idx_before;
                self.parse_expr_postfix()
            } else {
                Spanned::new(
                    CExpression::Cast {
                        type_name,
                        value: self.parse_expr_cast(),
                    },
                    start,
                    self.prev_token().loc,
                )
            }
        } else {
            self.parse_expr_unary()
        }
    }
    pub(crate) fn parse_expr_unary(&mut self) -> Spanned<CExpression> {
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
        let start = self.current_token().loc;
        let current_token = self.current_token();

        if current_token.original == "++" || current_token.original == "--" {
            // ++ unary-expression
            self.advance_idx();
            return Spanned::new(
                CExpression::PrefixIncrement {
                    value: self.parse_expr_unary(),
                    increment_type: if self.prev_token().original == "++" {
                        IncrementType::Increment
                    } else {
                        IncrementType::Decrement
                    },
                },
                start,
                self.prev_token().loc,
            );
        }
        if ["&", "*", "+", "-", "~", "!"].contains(&current_token.original.as_str()) {
            let op = match self.advance_idx().original.as_str() {
                "&" => UnaryOperator::REF,
                "*" => UnaryOperator::DEREF,
                "+" => UnaryOperator::VALUE,
                "-" => UnaryOperator::NEGATIVE,
                "~" => UnaryOperator::BITWISEINVERT,
                "!" => UnaryOperator::BOOLEANINVERT,
                _ => unreachable!(),
            };
            return Spanned::new(
                CExpression::Unary {
                    value: self.parse_expr_unary(),
                    unary_op: op,
                },
                start,
                self.prev_token().loc,
            );
        }
        if current_token.t_type == Keyword(CKeyword::SIZEOF) {
            self.advance_idx();
            if self.current_token().t_type == CTokenType::Punctuator
                && self.current_token().original == "("
            {
                // type name sizeof
                self.expect_type_and_string(CTokenType::Punctuator, "(");

                let type_name = self.parse_type_name();

                self.expect_type_and_string(CTokenType::Punctuator, ")");
                return Spanned::new(
                    CExpression::SizeOfType { type_name },
                    start,
                    self.prev_token().loc,
                );
            } else {
                // sizeof unary-expr
                return Spanned::new(
                    CExpression::SizeOf {
                        value: self.parse_expr_unary(),
                    },
                    start,
                    self.prev_token().loc,
                );
            }
        }
        if current_token.t_type == Keyword(CKeyword::ALIGNOF) {
            self.advance_idx();
            self.expect_type_and_string(CTokenType::Punctuator, "(");

            let type_name = self.parse_type_name();

            self.expect_type_and_string(CTokenType::Punctuator, ")");
            return Spanned::new(
                CExpression::AlignOfType { type_name },
                start,
                self.prev_token().loc,
            );
        }

        self.parse_expr_postfix()
    }
    pub(crate) fn parse_expr_postfix(&mut self) -> Spanned<CExpression> {
        /*
            primary-expression
            postfix-expression [ expression ]
            postfix-expression ( argument-expression-list opt ) // comma seperated list of assignment expression, only here
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
        let mut end; // = start.clone();

        let initial: Spanned<CExpression> = if self.current_token().t_type == CTokenType::Punctuator
            && self.current_token().original == "("
            && self.check_is_start_of_type_name(&self.next_token())
        {
            // ( type-name ) { initializer-list }
            self.advance_idx();
            let type_name = self.parse_type_name();

            self.expect_type_and_string(CTokenType::Punctuator, ")");
            self.expect_type_and_string(CTokenType::Punctuator, "{");
            self.idx -= 1;

            let initializer = self.parse_initializer();

            self.idx -= 1;
            self.expect_type_and_string(CTokenType::Punctuator, "}");

            Spanned::new(
                CExpression::TypeInitializer {
                    type_name,
                    initializer_list: initializer,
                },
                start.clone(),
                self.prev_token().loc,
            )
        } else {
            self.parse_expr_primary()
        };

        let mut result = initial;

        // check for '[' or '(' or '.' or '->' or '++' or '--' in a loop
        while self.current_token().t_type == Punctuator {
            let specific_punctuator = self.advance_idx().original;

            match specific_punctuator.as_str() {
                "[" => {
                    let index = self.parse_expression();
                    end = self.expect_type_and_string(Punctuator, "]").loc;
                    result = Spanned::new(
                        CExpression::ArraySubscription {
                            array: result,
                            index,
                        },
                        start.clone(),
                        end.clone(),
                    );
                }
                "(" => {
                    let args = self.parse_argument_expression_list();
                    end = self.expect_type_and_string(Punctuator, ")").loc;
                    result = Spanned::new(
                        CExpression::FunctionCall {
                            function: result,
                            arguments: args,
                        },
                        start.clone(),
                        end.clone(),
                    );
                }
                "." => {
                    let ident = Identifier {
                        identifier: self.expect_type(Identifier).original,
                    };
                    end = self.prev_token().loc;
                    result = Spanned::new(
                        CExpression::DirectMemberAccess {
                            to_access: result,
                            member: ident,
                        },
                        start.clone(),
                        end.clone(),
                    );
                }
                "->" => {
                    let ident = Identifier {
                        identifier: self.expect_type(Identifier).original,
                    };
                    end = self.prev_token().loc;
                    result = Spanned::new(
                        CExpression::IndirectMemberAccess {
                            to_access: result,
                            member: ident,
                        },
                        start.clone(),
                        end.clone(),
                    );
                }
                "++" | "--" => {
                    end = self.prev_token().loc;
                    result = Spanned::new(
                        CExpression::PostfixIncrement {
                            increment_type: if self.prev_token().original == "++" {
                                IncrementType::Increment
                            } else {
                                IncrementType::Decrement
                            },
                            value: result,
                        },
                        start,
                        end,
                    );
                    break;
                }
                _unknown => {
                    self.idx -= 1;
                    break;
                }
            }
        }

        result
    }
    pub(crate) fn parse_expr_primary(&mut self) -> Spanned<CExpression> {
        let current_token = self.current_token();
        match current_token.clone().t_type {
            crate::lexer::token_types::CTokenType::Keyword(keyword) => {
                // only GENERIC for generic Selection
                if keyword == CKeyword::GENERIC {
                    // generic selection
                    panic!("Generic Selection Expression Still unsuported!")
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
                Spanned::new(
                    CExpression::Identifier(Identifier {
                        identifier: current_token.original,
                    }),
                    current_token.loc.clone(),
                    current_token.loc,
                )
            }
            crate::lexer::token_types::CTokenType::Constant => {
                // return constant
                self.advance_idx();
                Spanned::new(
                    CExpression::Constant(Constant::Number(NumberLike {
                        from: current_token.original,
                    })),
                    current_token.loc.clone(),
                    current_token.loc,
                )
            }
            crate::lexer::token_types::CTokenType::StringLiteral => {
                self.advance_idx();
                Spanned::new(
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
                    let start = self.advance_idx().loc;
                    let expr = self.parse_expression();
                    let end = self.expect_type_and_string(CTokenType::Punctuator, ")").loc;

                    Spanned::new(CExpression::Paranthesised(expr), start, end)
                } else {
                    // unexpected punctuator -> panic and error?!
                    self.error_unexpected(
                        current_token,
                        "Expected only ( in primary Expression for parenthesised Expression",
                    );
                    unreachable!()
                }
            }
            Eof => unreachable!(),
        }
    }
}
