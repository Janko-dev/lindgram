

#[derive(Debug)]
pub enum Token {
    Char(char),
    Arrow,
    NewLine,
    Invalid(String),
}

pub struct Lexer<'a> {
    input: &'a [u8],
    at_end: bool
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { 
            input: input.as_bytes(),
            at_end: false
        }
    }

    fn next_u8(&mut self) -> &'a u8 {
        if self.input.len() == 0 {
            return &b'\n';
        }
        let token = &self.input[0];
        self.input = &self.input[1..];
        token
    }

    fn next_token(&mut self) -> Option<Token> {
        
        if self.at_end { return None; }

        if self.input.is_empty() {
            self.at_end = true;
            return Some(Token::NewLine);
        }

        let mut token = self.next_u8();

        while let b' ' | b'\t' = token {
            token = self.next_u8();
        }
        
        match token {
            b'=' => {
                match self.next_u8() {
                    b'>' => Some(Token::Arrow),
                    c => Some(Token::Invalid(format!("Expected '>' but got '{}'", *c as char)))
                }
            },
            b'\n' => Some(Token::NewLine),
            c => Some(Token::Char(*c as char))
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
