use std::path::Iter;

use super::declarations::*;
use super::expressions::*;
use super::*;

/*
(6.8) statement:
    labeled-statement
    compound-statement
    expression-statement
    selection-statement
    iteration-statement
    jump-statement
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Statement {
    Labeled(Box<LabeledStatement>),
    Compound(Box<CompoundStatement>),
    Expression(Box<ExpressionStatement>),
    Selection(Box<SelectionStatement>),
    Iteration(Box<IterationStatement>),
    Jump(Box<JumpStatement>),
}

/*
(6.8.1) labeled-statement:
    identifier : statement
    case constant-expression : statement
    default : statement
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LabeledStatement {
    ident: Identifier,
    body: Statement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SwitchLabeledStatement {
    Case {
        const_expr: ConstantExpression,
        body: Statement,
    },
    Default {
        body: Statement,
    },
}

/*
(6.8.2) compound-statement:
    { block-item-list opt }
(6.8.2) block-item-list:
    block-item
    block-item-list block-item
(6.8.2) block-item:
    declaration
    statemen
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CompoundStatement {
    body: Vec<BlockItem>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BlockItem {
    Declaration(Declaration),
    Statement(Statement),
}

/*
(6.8.3) expression-statement:
    expressionopt ;
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpressionStatement {
    body: Option<Expression>,
}

/*
(6.8.4) selection-statement:
    if ( expression ) statement
    if ( expression ) statement else statement
    switch ( expression ) statement
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SelectionStatement {
    If {
        cond: Expression,
        body: Statement,
    },
    IfElse {
        cond: Expression,
        body: Statement,
        else_body: Statement,
    },
    Switch {
        cond: Expression,
        body: Vec<SwitchLabeledStatement>,
    },
}

/*
(6.8.5) iteration-statement:
    while ( expression ) statement
    do statement while ( expression ) ;
    for ( expression opt ; expression opt ; expression opt ) statement
    for ( declaration expression opt ; expression opt ) statement
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IterationStatement {
    While {
        cond: Expression,
        body: Statement,
    },
    DoWhile {
        cond: Expression,
        body: Statement,
    },
    For {
        expr1: Option<Expression>,
        expr2: Option<Expression>,
        expr3: Option<Expression>,
        body: Statement,
    },
    ForDecl {
        declaration: Option<Declaration>,
        expr2: Option<Expression>,
        expr3: Option<Expression>,
        body: Statement,
    },
}

/*
(6.8.6) jump-statement:
    goto identifier ;
    continue ;
    break ;
    return expression opt ;
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum JumpStatement {
    Goto(Identifier),
    Continue,
    Break,
    Return(Option<Expression>),
}
