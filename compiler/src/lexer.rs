pub struct Lexer<'a> {
    content: &'a str,
    pos: usize,
}

#[derive(Debug)]
pub enum TokenKind {
    Unknown,

    Identifier,
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

            return Some(self.tokenize_unknown());
        }
    }
}
