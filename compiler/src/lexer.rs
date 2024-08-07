use std::{collections::HashMap, sync::OnceLock};

pub struct Lexer<'a> {
    content: &'a str,
    pos: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Unknown,
    Identifier,

    // Keywords
    Fn,
    Mut,
    If,
    Else,

    // Primitives
    Integer,
    Float,
    String,

    // Symbols and Operators
    LeftCurly,
    RightCurly,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Dot,
    Comma,
    Colon,
    Semi,
    DeclAssign,
    Assign,

    // Math Operators
    Plus,
    Minus,
    Times,
    DividedBy,

    // Conditional Operators
    Equals,
    Not,
    NotEquals,
    GreaterThan,
    GreaterOrEquals,
    LowerThan,
    LowerOrEquals,
    Or,
    And,

    // Bitwise Operator
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Self {
        Self { content, pos: 0 }
    }

    fn current(&self) -> Option<char> {
        self.content.chars().nth(self.pos)
    }

    fn advance(&mut self) -> Option<char> {
        self.pos += 1;
        self.content.chars().nth(self.pos)
    }

    fn is_number_token(c: char) -> bool {
        c.is_ascii_digit() || c == '.'
    }

    fn tokenize_number(&mut self) -> Token {
        let mut kind = TokenKind::Integer;
        let mut number_str = String::new();
        loop {
            let cur = self.current().unwrap();
            if cur == '.' {
                kind = TokenKind::Float;
            }

            number_str.push(cur);

            let Some(next) = self.advance() else { break };
            if !Self::is_number_token(next) {
                break;
            }
        }

        Token {
            kind,
            value: number_str,
        }
    }

    fn tokenize_unknown(&mut self) -> Token {
        let mut token = Token {
            kind: TokenKind::Unknown,
            value: String::from(self.current().unwrap()),
        };

        while let Some(c) = self.advance() {
            if c.is_whitespace() {
                break;
            }

            token.value.push(c);
        }

        token
    }

    fn tokenize_identifier(&mut self) -> Token {
        let mut token = Token {
            kind: TokenKind::Identifier,
            value: String::from(self.current().unwrap()),
        };

        while let Some(c) = self.advance() {
            if !c.is_ascii_alphanumeric() || c == '_' {
                break;
            }

            token.value.push(c);
        }

        match token.value.as_str() {
            "fn" => token.kind = TokenKind::Fn,
            "mut" => token.kind = TokenKind::Mut,
            "if" => token.kind = TokenKind::If,
            "else" => token.kind = TokenKind::Else,
            _ => {}
        }

        token
    }

    fn tokenize_string(&mut self) -> Token {
        let mut token = Token {
            kind: TokenKind::Unknown, // if the string is not closed, it will return an Unknown token
            value: String::new(),
        };

        while let Some(c) = self.advance() {
            if c == '"' {
                token.kind = TokenKind::String;
                self.advance();
                break;
            }

            token.value.push(c);
        }

        token
    }

    fn symbol_table() -> &'static HashMap<String, Token> {
        static TABLE: OnceLock<HashMap<String, Token>> = OnceLock::new();
        TABLE.get_or_init(|| {
            let token_pairs = [
                // Symbols and Operators
                ("{", TokenKind::LeftCurly),
                ("}", TokenKind::RightCurly),
                ("(", TokenKind::LeftParen),
                (")", TokenKind::RightParen),
                ("[", TokenKind::LeftBracket),
                ("]", TokenKind::RightBracket),
                (".", TokenKind::Dot),
                (",", TokenKind::Comma),
                (":", TokenKind::Colon),
                (";", TokenKind::Semi),
                (":=", TokenKind::DeclAssign),
                ("=", TokenKind::Assign),
                // Math Operators
                ("+", TokenKind::Plus),
                ("-", TokenKind::Minus),
                ("*", TokenKind::Times),
                ("/", TokenKind::DividedBy),
                // Conditional Operators
                ("==", TokenKind::Equals),
                ("!", TokenKind::Not),
                ("!=", TokenKind::NotEquals),
                ("||", TokenKind::Or),
                ("&&", TokenKind::And),
                (">", TokenKind::GreaterThan),
                (">=", TokenKind::GreaterOrEquals),
                ("<", TokenKind::LowerThan),
                ("<=", TokenKind::LowerOrEquals),
                // Bitwise Operators
                ("&", TokenKind::BitAnd),
                ("|", TokenKind::BitOr),
                ("^", TokenKind::BitXor),
                ("~", TokenKind::BitNot),
            ];
            let mut table = HashMap::new();

            for pair in token_pairs {
                table.insert(
                    pair.0.to_string(),
                    Token {
                        kind: pair.1,
                        value: pair.0.to_string(),
                    },
                );
            }

            table
        })
    }

    fn is_symbol_token(c: char) -> bool {
        static SYMBOL_CHARS: OnceLock<String> = OnceLock::new();
        let symbol_chars = SYMBOL_CHARS.get_or_init(|| {
            Self::symbol_table()
                .keys()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .join("")
        });

        symbol_chars.chars().find(|&search| search == c).is_some()
    }

    fn tokenize_symbol(&mut self) -> Token {
        let c = self.current().unwrap();
        let table = Self::symbol_table();

        let token = match c {
            ':' | '=' | '>' | '<' | '!' => {
                let regular = &table[&c.to_string()];

                let Some(next) = self.advance() else {
                    return regular.to_owned();
                };

                if next != '=' {
                    return regular.to_owned();
                }

                table[&format!("{}=", c)].clone()
            }

            '&' | '|' => {
                let regular = &table[&c.to_string()];

                let Some(next) = self.advance() else {
                    return regular.to_owned();
                };

                if next != c {
                    return regular.to_owned();
                }

                table[&format!("{}{}", c, c)].clone()
            }

            c => {
                let char_str = c.to_string();
                if table.contains_key(&char_str) {
                    table[&char_str].to_owned()
                } else {
                    Token {
                        kind: TokenKind::Unknown,
                        value: String::from(c),
                    }
                }
            }
        };

        self.advance();

        token
    }

    pub fn tokenize(&mut self) -> Option<Token> {
        loop {
            let c = self.current()?;
            if c.is_whitespace() {
                self.advance();
                continue;
            }

            if Self::is_number_token(c) {
                return Some(self.tokenize_number());
            }

            if c.is_ascii_alphabetic() || c == '_' {
                return Some(self.tokenize_identifier());
            }

            if c == '"' {
                return Some(self.tokenize_string());
            }

            if Self::is_symbol_token(c) {
                return Some(self.tokenize_symbol());
            }

            return Some(self.tokenize_unknown());
        }
    }
}
