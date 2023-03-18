use std::{collections::HashMap, f64::consts::PI};

use super::lexer::*;

pub struct Model {
    pub rules: HashMap<char, String>,
    pub error_stack: Vec<String>,
    pub axiom: String,
    pub new_axiom: String,
    
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

    pub fn render(&mut self, width: usize, height: usize, xoff: i32, yoff: i32, line_len: i32, angle: f64) -> Vec<u8> {
        
        let mut renderer = Renderer::new(xoff, yoff, width, height);

        for ax in self.axiom.chars() {

            match ax {
                'F' => {
                    renderer.forward(line_len);
                    renderer.walk(line_len);
                },
                'f' => {
                    renderer.walk(line_len);
                },
                '+' => {
                    renderer.rotate(angle);
                },
                '-' => {
                    renderer.rotate(-angle);
                },
                '[' => {
                    renderer.push();
                },
                ']' => {
                    renderer.pop();
                },
                _ => {}
            }
        }

        renderer.pixels
    }

}

pub struct Renderer {
    pub pixels: Vec<u8>,
    xoff: i32,
    yoff: i32,
    angle: f64,
    width: usize, 
    height: usize,
    stack: Vec<(i32, i32, f64)>
}

impl Renderer {
    fn new(xoff: i32, yoff: i32, width: usize, height: usize) -> Self {
        Self { 
            pixels: vec![255; width*height*4], 
            xoff, 
            yoff,
            angle: -PI / 2.,
            width, 
            height,
            stack: vec![]
        }
    }

    fn push(&mut self) {
        self.stack.push((self.xoff, self.yoff, self.angle));
    }

    fn pop(&mut self) {
        let (xoff, yoff, angle) = match self.stack.pop() {
            Some(x) => x,
            None => (0, 0, 0.0)
        };

        self.angle = angle;
        self.xoff = xoff;
        self.yoff = yoff;
    }

    fn rotate(&mut self, a: f64) {
        self.angle += a;
    }

    fn walk(&mut self, dist: i32) {
        self.xoff += (dist as f64 * self.angle.cos()).round() as i32;
        self.yoff += (dist as f64 * self.angle.sin()).round() as i32;
    }

    fn forward(&mut self, dist: i32) {

        let x1 = self.xoff;
        let y1 = self.yoff;

        let x2 = (dist as f64 * self.angle.cos()).round() as i32 + self.xoff;
        let y2 = (dist as f64 * self.angle.sin()).round() as i32 + self.yoff;

        let dx = x2 - x1;
        let dy = y2 - y1;

        if dx == 0 && dy == 0 {
            return;
        }

        if dx.abs() > dy.abs() {
            let (mut x1, mut x2, y1, y2) = if x1 > x2 {
                (x2, x1, y2, y1)
            } else {
                (x1, x2, y1, y2)
            };

            if x1 < 0 { x1 = 0; }
            if x2 >= self.width as i32 { x2 = self.width as i32 - 1; }
            
            for x in x1..x2 {
                let y = dy * (x - x1)/dx + y1;
                if 0 <= y && y < self.height as i32 {
                    let index = ((y)*self.width as i32 + (x)) as usize * 4;
                    self.pixels[index+0] = 0;
                    self.pixels[index+1] = 0;
                    self.pixels[index+2] = 0;
                    self.pixels[index+3] = 0;
                }
            }
        } else {
            let (x1, x2, mut y1, mut y2) = if y1 > y2 {
                (x2, x1, y2, y1)
            } else {
                (x1, x2, y1, y2)
            };

            if y1 < 0 { y1 = 0; }
            if y2 >= self.height as i32 { y2 = self.height as i32 - 1; }

            // println!("ys: {y1}, {y2}");
            for y in y1..y2 {
                let x = dx * (y - y1)/dy + x1;
                if 0 <= x && x < self.width as i32 {
                    let index = ((y)*self.width as i32 + (x)) as usize * 4;
                    self.pixels[index+0] = 0;
                    self.pixels[index+1] = 0;
                    self.pixels[index+2] = 0;
                    self.pixels[index+3] = 0;
                }
            }
        }

    }
}

