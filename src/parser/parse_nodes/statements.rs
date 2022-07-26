use crate::lexer::token_types::CKeyword;
use crate::lexer::token_types::CTokenType;
use crate::parser::CParser;
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
    Labeled{
        label: Identifier,
        body: Spanned<Statement>,
    },
    SwitchCase{

    },
    SwitchDefault{

    },
    Compound(Vec<Spanned<Self>>),
    CExpression(Spanned<CExpression>),
    NoneExpr,
    If{

    },
    Switch{

    },
    While{
        /// differ do-while and while
        /// 0 -> while
        /// 1 -> do-while
        while_type: bool,
    },
    For{

    },
    Goto(),
    Continue,
    Break,
    Return(),
}

impl CParser{
    pub(crate) fn parse_statement(&mut self) -> Statement{
        // differentiate the different statement types:
        // labeled -> ident : -> case const-expr : -> default :
        // compound -> { ... } 
        // expression -> opt-expression ;
        // selection -> if ( -> switch (
        // iteration -> while ( -> do -> for
        // jump -> goto -> continue -> break -> return
        match self.current_token().t_type{
            CTokenType::Keyword(keyword) => {
                // case,default -> labeled
                // if,switch -> selection
                // while,do,for -> iteration
                // goto,continue,break,return -> jump
                // rest to expression?
                #[allow(clippy::if_same_then_else)]
                if [CKeyword::CASE,CKeyword::DEFAULT].contains(&keyword){
                    // labeled
                    // return Statement::Labeled(self.parse_labeled_statement());
                } else if [CKeyword::IF,CKeyword::SWITCH].contains(&keyword){
                    // selection
                } else if [CKeyword::WHILE,CKeyword::DO,CKeyword::FOR].contains(&keyword){
                    // iteration
                } else if [CKeyword::CASE,CKeyword::DEFAULT].contains(&keyword){
                    // labeled
                } 
                todo!()
            },
            CTokenType::Identifier => {
                // labeled or expression
                todo!()
            },
            CTokenType::Constant => {
                // expression
                todo!()
            },
            CTokenType::StringLiteral => {
                // expression
                todo!()
            },
            CTokenType::Punctuator => {
                // ; -> expression
                // { -> compound-statement
                todo!()
            },
            CTokenType::Eof => {
                self.error_unexpected(self.current_token(), "unexpected End of File in parse_statement");
                unreachable!()
            },
        }
    }
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
    body: Vec<Spanned<BlockItem>>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum BlockItem {
    Statement(Statement),
    Declaration(Spanned<Declaration>)
}

/*
(6.8.3) expression-statement:
    expression-opt ;
*/

/*
(6.8.4) selection-statement:
    if ( expression ) statement
    if ( expression ) statement else statement
    switch ( expression ) statement
*/
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SelectionStatement {
    If {
        cond: Spanned<CExpression>,
        body: Statement,
    },
    IfElse {
        cond: Spanned<CExpression>,
        body: Statement,
        else_body: Statement,
    },
    Switch {
        cond: Spanned<CExpression>,
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
        cond: Spanned<CExpression>,
        body: Statement,
    },
    DoWhile {
        cond: Spanned<CExpression>,
        body: Statement,
    },
    For {
        expr1: Option<Spanned<CExpression>>,
        expr2: Option<Spanned<CExpression>>,
        expr3: Option<Spanned<CExpression>>,
        body: Statement,
    },
    ForDecl {
        expr2: Option<Spanned<CExpression>>,
        expr3: Option<Spanned<CExpression>>,
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
    Return(Option<Spanned<CExpression>>),
}
