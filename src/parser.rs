pub struct Parser {
    text: String,
}

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    LBrace,
    RBrace
}

impl Parser {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn parse(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = self.text.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '{' => tokens.push(Token::LBrace),
                '}' => tokens.push(Token::RBrace),
                c if c.is_alphabetic() => {
                    let mut ident = String::new();
                    ident.push(c);

                    while let Some(&next) = chars.peek() {
                        if next.is_alphanumeric() || next == '_' {
                            ident.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    tokens.push(Token::Identifier(ident));
                },
                _ => {}
            }
        }

        tokens
    }
}
