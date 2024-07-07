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
            if !next.is_ascii_digit() && next != '.' {
                break;
            }
        }

        Token {
            kind,
            value: number_str,
        }
    }

    pub fn tokenize(&mut self) -> Option<Token> {
        loop {
            let c = self.current()?;
            if c.is_whitespace() {
                self.advance();
                continue;
            }

            if c.is_ascii_digit() || c == '.' {
                return Some(self.tokenize_number());
            }

            self.advance();
        }
    }
}
