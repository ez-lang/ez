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
            if !c.is_ascii_alphanumeric() {
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

            if c.is_ascii_alphabetic() {
                return Some(self.tokenize_identifier());
            }

            if c == '"' {
                return Some(self.tokenize_string());
            }

            return Some(self.tokenize_unknown());
        }
    }
}
