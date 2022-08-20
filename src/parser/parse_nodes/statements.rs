use crate::lexer::token_types::CKeyword;
use crate::lexer::token_types::CTokenType;
use crate::lexer::CToken;
use crate::parser::span::Spanned;
use crate::parser::CParser;

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
    Labeled {
        label: Identifier,
        body: Spanned<Statement>,
    },
    SwitchCase {
        const_expr: ConstantExpression,
        statement: Spanned<Self>,
    },
    SwitchDefault {
        statement: Spanned<Self>,
    },
    Compound(Vec<CompoundItem>),
    CExpression(Spanned<CExpression>),
    NoneExpr,
    If {
        controlling_expr: Spanned<CExpression>,
        true_body: Spanned<Self>,
        else_body: Option<Spanned<Self>>,
    },
    Switch {
        controlling_expr: Spanned<CExpression>,
        body: Spanned<Self>,
    },
    While {
        /// differ do-while and while
        /// 0 -> while
        /// 1 -> do-while
        while_type: bool,
        controlling_expr: Spanned<CExpression>,
        body: Spanned<Self>,
    },
    For {
        /// either decl clause or expr clause
        decl_clause: Option<Spanned<Declaration>>,
        /// either decl clause or expr clause
        expr_clause: Option<Spanned<CExpression>>,
        controlling_expr: Option<Spanned<CExpression>>,
        after_expr: Option<Spanned<CExpression>>,
        body: Spanned<Statement>,
    },
    Goto(Identifier),
    Continue,
    Break,
    Return(Option<Spanned<CExpression>>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum CompoundItem {
    Statement(Spanned<Statement>),
    Declaration(Spanned<Declaration>),
}

impl CParser {
    pub(crate) fn parse_statement(&mut self) -> Spanned<Statement> {
        let start = self.current_token().loc;
        // differentiate the different statement types:
        // labeled -> ident : -> case const-expr : -> default :
        // compound -> { ... }
        // expression -> opt-expression ;
        // selection -> if ( -> switch (
        // iteration -> while ( -> do -> for
        // jump -> goto -> continue -> break -> return
        match self.current_token().t_type {
            CTokenType::Keyword(keyword) => {
                // case,default -> labeled
                // if,switch -> selection
                // while,do,for -> iteration
                // goto,continue,break,return -> jump
                // rest to expression?
                if [CKeyword::CASE, CKeyword::DEFAULT].contains(&keyword) {
                    // labeled
                    if self.advance_idx().t_type == CTokenType::Keyword(CKeyword::CASE) {
                        let const_expr = self.parse_constant_expr();
                        self.expect_type_and_string(CTokenType::Punctuator, ":");
                        Spanned::new(
                            Statement::SwitchCase {
                                const_expr,
                                statement: self.parse_statement(),
                            },
                            start,
                            self.prev_token().loc,
                        )
                    } else {
                        self.expect_type_and_string(CTokenType::Punctuator, ":");
                        Spanned::new(
                            Statement::SwitchDefault {
                                statement: self.parse_statement(),
                            },
                            start,
                            self.prev_token().loc,
                        )
                    }
                } else if [CKeyword::IF, CKeyword::SWITCH].contains(&keyword) {
                    // selection
                    if self.advance_idx().t_type == CTokenType::Keyword(CKeyword::IF) {
                        self.expect_type_and_string(CTokenType::Punctuator, "(");
                        let controlling_expr = self.parse_expression();
                        self.expect_type_and_string(CTokenType::Punctuator, ")");

                        let true_body = self.parse_statement();
                        let else_body =
                            if self.current_token().t_type == CTokenType::Keyword(CKeyword::ELSE) {
                                self.advance_idx();
                                Some(self.parse_statement())
                            } else {
                                None
                            };

                        Spanned::new(
                            Statement::If {
                                controlling_expr,
                                true_body,
                                else_body,
                            },
                            start,
                            self.prev_token().loc,
                        )
                    } else {
                        self.expect_type_and_string(CTokenType::Punctuator, "(");
                        let controlling_expr = self.parse_expression();
                        self.expect_type_and_string(CTokenType::Punctuator, ")");

                        let body = self.parse_statement();

                        Spanned::new(
                            Statement::Switch {
                                controlling_expr,
                                body,
                            },
                            start,
                            self.prev_token().loc,
                        )
                    }
                } else if [CKeyword::WHILE, CKeyword::DO, CKeyword::FOR].contains(&keyword) {
                    // iteration
                    self.advance_idx();
                    if CKeyword::WHILE == keyword {
                        self.expect_type_and_string(CTokenType::Punctuator, "(");

                        let controlling_expr = self.parse_expression();

                        self.expect_type_and_string(CTokenType::Punctuator, ")");

                        let body = self.parse_statement();

                        Spanned::new(
                            Statement::While {
                                while_type: false,
                                controlling_expr,
                                body,
                            },
                            start,
                            self.prev_token().loc,
                        )
                    } else if CKeyword::DO == keyword {
                        let body = self.parse_statement();
                        self.expect_type(CTokenType::Keyword(CKeyword::WHILE));
                        self.expect_type_and_string(CTokenType::Punctuator, "(");

                        let controlling_expr = self.parse_expression();

                        self.expect_type_and_string(CTokenType::Punctuator, ")");
                        self.expect_type_and_string(CTokenType::Punctuator, ";");

                        Spanned::new(
                            Statement::While {
                                while_type: true,
                                controlling_expr,
                                body,
                            },
                            start,
                            self.prev_token().loc,
                        )
                    } else {
                        // for loop TODO
                        self.expect_type_and_string(CTokenType::Punctuator, "(");
                        let decl_clause = if self.is_start_of_declaration(self.current_token()) {
                            Some(self.parse_declaration())
                        } else {
                            None
                        };

                        let expr_clause = if decl_clause.is_some() {
                            None
                        } else if self.current_token().t_type == CTokenType::Punctuator
                            && self.current_token().original == ";"
                        {
                            self.advance_idx();
                            None
                        } else {
                            let expr = self.parse_expression();
                            self.expect_type_and_string(CTokenType::Punctuator, ";");
                            Some(expr)
                        };

                        let controlling_expr = if self.current_token().t_type
                            == CTokenType::Punctuator
                            && self.current_token().original == ";"
                        {
                            self.advance_idx();
                            None
                        } else {
                            let expr = self.parse_expression();
                            self.expect_type_and_string(CTokenType::Punctuator, ";");
                            Some(expr)
                        };

                        let after_expr = if self.current_token().t_type == CTokenType::Punctuator
                            && self.current_token().original == ")"
                        {
                            None
                        } else {
                            Some(self.parse_expression())
                        };

                        self.expect_type_and_string(CTokenType::Punctuator, ")");

                        let body = self.parse_statement();

                        Spanned::new(
                            Statement::For {
                                decl_clause,
                                expr_clause,
                                controlling_expr,
                                after_expr,
                                body,
                            },
                            start,
                            self.prev_token().loc,
                        )
                    }
                } else if CKeyword::GOTO == keyword {
                    self.advance_idx();
                    let ident = Identifier {
                        identifier: self.expect_type(CTokenType::Identifier).original,
                    };
                    self.expect_type_and_string(CTokenType::Punctuator, ";");
                    Spanned::new(Statement::Goto(ident), start, self.prev_token().loc)
                } else if CKeyword::CONTINUE == keyword {
                    self.advance_idx();
                    self.expect_type_and_string(CTokenType::Punctuator, ";");
                    Spanned::new(Statement::Continue, start, self.prev_token().loc)
                } else if CKeyword::BREAK == keyword {
                    self.advance_idx();
                    self.expect_type_and_string(CTokenType::Punctuator, ";");
                    Spanned::new(Statement::Break, start, self.prev_token().loc)
                } else if CKeyword::RETURN == keyword {
                    self.advance_idx();

                    if self.current_token().t_type == CTokenType::Punctuator
                        && self.current_token().original == ";"
                    {
                        self.advance_idx();
                        Spanned::new(Statement::Return(None), start, self.prev_token().loc)
                    } else {
                        let return_expr = self.parse_expression();
                        self.expect_type_and_string(CTokenType::Punctuator, ";");
                        Spanned::new(
                            Statement::Return(Some(return_expr)),
                            start,
                            self.prev_token().loc,
                        )
                    }
                } else {
                    // Jank, but i don't know a better way right now! ;)
                    self.error_unexpected(
                        self.current_token(),
                        "unknown keyword in statement check",
                    );
                    unreachable!()
                }
            }
            CTokenType::Identifier => {
                // labeled or expression
                if self.next_token().t_type == CTokenType::Punctuator
                    && self.next_token().original == ":"
                {
                    Spanned::new(
                        Statement::Labeled {
                            label: Identifier {
                                identifier: self.advance_idx().original,
                            },
                            body: {
                                self.advance_idx(); // remove the :
                                self.parse_statement()
                            },
                        },
                        start,
                        self.prev_token().loc,
                    )
                } else {
                    let expr = self.parse_expression();
                    self.expect_type_and_string(CTokenType::Punctuator, ";");
                    Spanned::new(Statement::CExpression(expr), start, self.prev_token().loc)
                }
            }
            CTokenType::Constant => {
                // expression
                let expr = self.parse_expression();
                self.expect_type_and_string(CTokenType::Punctuator, ";");
                Spanned::new(Statement::CExpression(expr), start, self.prev_token().loc)
            }
            CTokenType::StringLiteral => {
                // expression
                let expr = self.parse_expression();
                self.expect_type_and_string(CTokenType::Punctuator, ";");
                Spanned::new(Statement::CExpression(expr), start, self.prev_token().loc)
            }
            CTokenType::Punctuator => {
                // ; -> expression
                // { -> compound-statement
                // else expr?
                if self.current_token().original == ";" {
                    Spanned::new(Statement::NoneExpr, start, self.advance_idx().loc)
                } else if self.current_token().original == "{" {
                    // compund expr
                    // block item is either declaration or statement
                    self.push_typedef_scope();
                    let mut compound_statement_list = vec![];
                    self.advance_idx();
                    while !(self.current_token().t_type == CTokenType::Punctuator
                        && self.current_token().original == "}")
                    {
                        if self.is_start_of_declaration(self.current_token()) {
                            compound_statement_list
                                .push(CompoundItem::Declaration(self.parse_declaration()));
                        } else {
                            compound_statement_list
                                .push(CompoundItem::Statement(self.parse_statement()));
                        }
                    }
                    self.advance_idx();
                    self.pop_typedef_scope();
                    Spanned::new(
                        Statement::Compound(compound_statement_list),
                        start,
                        self.prev_token().loc,
                    )
                } else {
                    let expr = self.parse_expression();
                    self.expect_type_and_string(CTokenType::Punctuator, ";");
                    Spanned::new(Statement::CExpression(expr), start, self.prev_token().loc)
                }
            }
            CTokenType::Eof => {
                self.error_unexpected(
                    self.current_token(),
                    "unexpected End of File in parse_statement",
                );
                unreachable!()
            }
        }
    }
}

/*
(6.8.1) labeled-statement:
    identifier : statement
    case constant-expression : statement
    default : statement
*/

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

impl CParser {
    pub(crate) fn is_start_of_declaration(&mut self, token: CToken) -> bool {
        match token.t_type {
            CTokenType::Keyword(keyword) => {
                use CKeyword::*;
                [TYPEDEF, EXTERN, STATIC, THREAD_LOCAL, AUTO, REGISTER].contains(&keyword)
                    || [
                        VOID, CHAR, SHORT, INT, LONG, DOUBLE, SIGNED, UNSIGNED, BOOL, COMPLEX,
                    ]
                    .contains(&keyword)
                    || [ATOMIC, STRUCT, UNION, ENUM].contains(&keyword)
                    || [CONST, RESTRICT, VOLATILE, ATOMIC].contains(&keyword)
                    || [INLINE, NORETURN].contains(&keyword)
                    || [ALIGNAS].contains(&keyword)
                    || [STATIC_ASSERT].contains(&keyword)
            }
            CTokenType::Identifier => self.is_typedef(&token.original),
            CTokenType::Constant => false,
            CTokenType::StringLiteral => false,
            CTokenType::Punctuator => false,
            CTokenType::Eof => false,
        }
    }
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

/*
(6.8.5) iteration-statement:
    while ( expression ) statement
    do statement while ( expression ) ;
    for ( expression opt ; expression opt ; expression opt ) statement
    for ( declaration expression opt ; expression opt ) statement
*/

/*
(6.8.6) jump-statement:
    goto identifier ;
    continue ;
    break ;
    return expression opt ;
*/
