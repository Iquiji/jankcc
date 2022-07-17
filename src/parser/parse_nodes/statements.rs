use crate::parser::span::Spanned;

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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Statement {
    Labeled(Box<LabeledStatement>),
    Compound(Box<CompoundStatement>),
    CExpression(Box<ExpressionStatement>),
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LabeledStatement {
    ident: Identifier,
    body: Statement,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CompoundStatement {
    body: Vec<BlockItem>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum BlockItem {
    Statement(Statement),
}

/*
(6.8.3) expression-statement:
    expressionopt ;
*/
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExpressionStatement {
    body: Option<Box<Spanned<CExpression>>>,
}

/*
(6.8.4) selection-statement:
    if ( expression ) statement
    if ( expression ) statement else statement
    switch ( expression ) statement
*/
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SelectionStatement {
    If {
        cond: Box<Spanned<CExpression>>,
        body: Statement,
    },
    IfElse {
        cond: Box<Spanned<CExpression>>,
        body: Statement,
        else_body: Statement,
    },
    Switch {
        cond: Box<Spanned<CExpression>>,
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum IterationStatement {
    While {
        cond: Box<Spanned<CExpression>>,
        body: Statement,
    },
    DoWhile {
        cond: Box<Spanned<CExpression>>,
        body: Statement,
    },
    For {
        expr1: Option<Box<Spanned<CExpression>>>,
        expr2: Option<Box<Spanned<CExpression>>>,
        expr3: Option<Box<Spanned<CExpression>>>,
        body: Statement,
    },
    ForDecl {
        expr2: Option<Box<Spanned<CExpression>>>,
        expr3: Option<Box<Spanned<CExpression>>>,
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum JumpStatement {
    Goto(Identifier),
    Continue,
    Break,
    Return(Option<Box<Spanned<CExpression>>>),
}
