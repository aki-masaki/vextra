use crate::ast::ASTNode;
use std::iter::Peekable;

pub struct Parser {
    text: String,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Literal(String),
    LBrace,
    RBrace,
    Gt,
    Colon,
    EOF,
}

impl Parser {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = self.text.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '{' => tokens.push(Token::LBrace),
                '}' => tokens.push(Token::RBrace),
                '>' => tokens.push(Token::Gt),
                ':' => tokens.push(Token::Colon),
                c if c == '"' => {
                    let mut literal = String::new();

                    while let Some(&next) = chars.peek() {
                        if next != '"' {
                            literal.push(chars.next().unwrap());
                        } else {
                            chars.next();

                            break;
                        }
                    }

                    tokens.push(Token::Literal(literal));
                }
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
                }
                _ => {}
            }
        }

        tokens.push(Token::EOF);

        tokens
    }

    pub fn parse(&self, tokens: &[Token]) -> ASTNode {
        let mut tokens = tokens.iter().peekable();

        match tokens.next() {
            Some(Token::Identifier(id)) if id == "app" => {
                // todo: Implement a `expect_token` function

                let mut children = Vec::new();

                while let Some(token) = tokens.peek() {
                    if matches!(token, Token::RBrace) {
                        tokens.next();
                        break;
                    }

                    match token {
                        Token::Gt => {
                            tokens.next();

                            let mut value = String::new();

                            let name = if let Some(Token::Identifier(id)) = tokens.next() {
                                id.clone()
                            } else {
                                panic!("Expected identifier after '>'");
                            };

                            if let Some(Token::Colon) = tokens.peek() {
                                tokens.next();

                                value = if let Some(Token::Literal(val)) = tokens.next() {
                                    val.clone()
                                } else {
                                    panic!("Expected string literal after :");
                                };
                            }

                            let node = ASTNode::Element { name, value };
                            children.push(node);
                        }
                        _ => {
                            tokens.next();
                        }
                    }
                }

                ASTNode::App { children }
            }
            _ => panic!("Expected 'app'"),
        }
    }
}
