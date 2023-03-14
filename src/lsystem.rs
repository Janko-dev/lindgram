use std::collections::HashMap;

use super::lexer::*;

pub struct Model {
    pub rules: HashMap<char, String>,
    pub error_stack: Vec<String>,
    pub axiom: String,
    pub new_axiom: String
}

impl Model {
    pub fn new(initial_axiom: &str) -> Self {
        Self { 
            rules: Default::default(),
            error_stack: vec![],
            axiom: initial_axiom.to_string(),
            new_axiom: String::new(),
        }
    }

    pub fn interpret(&mut self, input: &str) {
        let mut lexer = Lexer::new(input).peekable();

        let mut lhs: char = Default::default();
        let mut rhs = String::new();

        loop {
            // Check for lhs non terminal 
            if let Some(Token::Char(c)) = lexer.peek() {
                lhs = *c;
                lexer.next();
                if let Some(Token::Arrow) = lexer.peek() {
                    lexer.next();
                    // accumulate rhs chars
                    while let Some(Token::Char(c)) = lexer.peek() {
                        rhs.push(*c);
                        lexer.next();
                    }
    
                    // expect newline to terminate expression
                    if let Some(Token::NewLine) = lexer.next(){
                        self.rules.insert(lhs, rhs);
    
                        rhs = String::new();
                    }
                } else {
                    // Collect invalid tokens
                    for token in lexer {
                        if let Token::Invalid(msg) = token {
                            self.error_stack.push(msg);
                        }
                    }
                    break;
                }
            } else {
                break;
            }
        }
    }

    pub fn generate(&mut self, n: i32) {
        if n == 0 {
            return;
        }
        
        for ch in self.axiom.chars() {

            if self.rules.contains_key(&ch) {
                self.new_axiom.push_str(&self.rules[&ch]);
            } else {
                self.new_axiom.push(ch);
            }
        }

        self.axiom = self.new_axiom.clone();
        self.new_axiom = String::new();
        self.generate(n-1);
    }

}

