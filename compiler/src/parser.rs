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
pub enum Expr {
    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: TokenKind,
    },

    Declaration {
        identifier: String,
        value: Box<Expr>,
    },

    Number(f64),
    String(String),
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

    fn parse_value(&mut self) -> Result<Expr, ParseError> {
        let token = self.current().clone().unwrap();
        match token.kind {
            // For now all numbers will be the same type
            TokenKind::Integer | TokenKind::Float => {
                let Ok(number) = token.value.parse::<f64>() else {
                    return Err(ParseError::InvalidNumber(token));
                };

                Ok(Expr::Number(number))
            }

            TokenKind::String => Ok(Expr::String(token.value)),

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

                Ok(Expr::Declaration {
                    identifier: ident.value,
                    value: Box::new(value_expr),
                })
            }

            _ => Err(ParseError::UnexpectedToken(next.clone())),
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let Some(token) = self.advance() else {
            return Err(ParseError::NoMoreTokens);
        };

        match token.kind {
            TokenKind::Identifier => self.parse_identifier(),
            _ => Err(ParseError::UnexpectedToken(token.clone())),
        }
    }
}
