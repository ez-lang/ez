pub struct Lexer<'a> {
    content: &'a str,
    pos: usize,
}

#[derive(Debug)]
pub enum TokenKind {
    Unknown,
    Identifier,

    // Keywords
    Mut,

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
    Assignment,
}

#[derive(Debug)]
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
            "mut" => token.kind = TokenKind::Mut,
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

    fn is_symbol_token(c: char) -> bool {
        c == '{'
            || c == '}'
            || c == '('
            || c == ')'
            || c == '['
            || c == ']'
            || c == '.'
            || c == ','
            || c == ':'
            || c == '='
            || c == ';'
    }

    // TODO: Ensure all `is_symbol_token`s are handled here using some sort of type safety
    fn tokenize_symbol(&mut self) -> Token {
        let token = match self.current().unwrap() {
            '{' => Token {
                kind: TokenKind::LeftCurly,
                value: String::from('{'),
            },

            '}' => Token {
                kind: TokenKind::RightCurly,
                value: String::from('}'),
            },

            '(' => Token {
                kind: TokenKind::LeftParen,
                value: String::from('('),
            },

            ')' => Token {
                kind: TokenKind::RightParen,
                value: String::from(')'),
            },

            '[' => Token {
                kind: TokenKind::LeftBracket,
                value: String::from('['),
            },

            ']' => Token {
                kind: TokenKind::RightBracket,
                value: String::from(']'),
            },

            '.' => Token {
                kind: TokenKind::Dot,
                value: String::from('.'),
            },

            ',' => Token {
                kind: TokenKind::Comma,
                value: String::from(','),
            },

            ':' => {
                let regular_colon_token = Token {
                    kind: TokenKind::Colon,
                    value: String::from(':'),
                };

                let Some(next) = self.advance() else {
                    return regular_colon_token;
                };

                if next != '=' {
                    return regular_colon_token;
                }

                Token {
                    kind: TokenKind::Assignment,
                    value: String::from(":="),
                }
            }

            ';' => Token {
                kind: TokenKind::Semi,
                value: String::from(';'),
            },

            c => Token {
                kind: TokenKind::Unknown,
                value: String::from(c),
            },
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
