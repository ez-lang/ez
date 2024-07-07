use crate::lexer::{Lexer, Token, TokenKind};

pub struct Parser<'a> {
    current: Option<Token>,
    lexer: Lexer<'a>,
}

#[derive(Debug)]
pub enum ParseError {
    NoMoreTokens, // "soft" error (will happen at the EOF)
    MissingTokenAfter(Token),
    UnexpectedToken(Token),
    InvalidNumber(Token),
}

#[derive(Debug)]
pub struct Param {
    identifier: String,
    basetype: BaseType,
}

#[derive(Debug)]
pub enum BaseType {
    Void,
    Number,
    String,
    Function {
        params: Vec<Param>,
        return_type: Box<BaseType>,
    },
}

#[derive(Debug)]
pub enum ValueExpr {
    Number(f64),
    String(String),
    Function {
        params: Vec<Param>,
        return_type: BaseType,
        body: Vec<Expr>,
    },
}

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: TokenKind,
    },

    Declaration {
        identifier: String,
        value: Box<ValueExpr>,
    },

    Block {
        body: Vec<Expr>,
    },
}

impl Expr {
    fn codegen(&self) {}
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            lexer: Lexer::new(content),
            current: None,
        }
    }

    fn current(&self) -> &Option<Token> {
        &self.current
    }

    fn advance(&mut self) -> &Option<Token> {
        self.current = self.lexer.tokenize();

        &self.current
    }

    fn parse_function(&mut self) -> Result<ValueExpr, ParseError> {
        // lets ignore arguments for now!
        let mut body: Vec<Expr> = vec![];

        // go to left curly
        let mut previous = self.current().clone().unwrap();
        loop {
            let Some(token) = self.advance() else {
                return Err(ParseError::MissingTokenAfter(previous));
            };

            if token.kind == TokenKind::LeftCurly {
                break;
            }

            previous = token.to_owned();
        }

        // loop until right curly
        loop {
            let Some(token) = self.advance() else {
                return Err(ParseError::MissingTokenAfter(previous));
            };

            if token.kind == TokenKind::RightCurly {
                self.advance();
                break;
            }

            previous = token.to_owned();
            body.push(self.parse()?);
        }

        Ok(ValueExpr::Function {
            params: vec![],
            return_type: BaseType::Void,
            body,
        })
    }

    fn parse_value(&mut self) -> Result<ValueExpr, ParseError> {
        let token = self.current().clone().unwrap();
        match token.kind {
            // For now all numbers will be the same type
            TokenKind::Integer | TokenKind::Float => {
                let Ok(number) = token.value.parse::<f64>() else {
                    return Err(ParseError::InvalidNumber(token));
                };

                self.advance();

                Ok(ValueExpr::Number(number))
            }

            TokenKind::String => {
                self.advance();
                Ok(ValueExpr::String(token.value))
            }

            TokenKind::Fn => self.parse_function(),

            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    fn parse_identifier(&mut self) -> Result<Expr, ParseError> {
        let ident = self.current().to_owned().unwrap();
        let Some(next) = self.advance().to_owned() else {
            return Err(ParseError::MissingTokenAfter(ident.clone()));
        };

        match next.kind {
            TokenKind::DeclAssign => {
                if self.advance().is_none() {
                    return Err(ParseError::MissingTokenAfter(next));
                };

                let value_expr = self.parse_value()?;

                match value_expr {
                    ValueExpr::Function { .. } => {}
                    _ => {
                        let Some(token) = self.current() else {
                            return Err(ParseError::MissingTokenAfter(next));
                        };

                        if token.kind != TokenKind::Semi {
                            return Err(ParseError::UnexpectedToken(token.to_owned()));
                        }
                    }
                }

                Ok(Expr::Declaration {
                    identifier: ident.value,
                    value: Box::new(value_expr),
                })
            }

            _ => Err(ParseError::UnexpectedToken(next.clone())),
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        if self.current().is_none() {
            self.advance();
        }

        let Some(token) = self.current() else {
            return Err(ParseError::NoMoreTokens);
        };

        match token.kind {
            TokenKind::Identifier => self.parse_identifier(),
            _ => Err(ParseError::UnexpectedToken(token.clone())),
        }
    }
}
