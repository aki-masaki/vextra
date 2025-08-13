use crate::ast::ASTNode;
use crate::ast::Styles;
use std::collections::HashMap;
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
    Eof,
    Hash,
    Comma,
    Number(i16),
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
                '#' => tokens.push(Token::Hash),
                ',' => tokens.push(Token::Comma),
                '"' => {
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
                c if c.is_numeric() => {
                    let mut number = String::new();
                    number.push(c);

                    while let Some(&next) = chars.peek() {
                        if next.is_numeric() {
                            number.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    tokens.push(Token::Number(number.parse::<i16>().unwrap()));
                }
                _ => {}
            }
        }

        tokens.push(Token::Eof);

        tokens
    }

    pub fn parse(tokens: &[Token]) -> ASTNode {
        let mut tokens = tokens.iter().peekable();

        match tokens.next() {
            Some(Token::Identifier(id)) if id == "app" => {
                // todo: Implement an expect_token function

                let mut children = Vec::new();
                let mut title = String::from("Website");

                while let Some(token) = tokens.peek().cloned() {
                    if let Token::Colon = token {
                        tokens.next();

                        if let Some(Token::Literal(value)) = tokens.next() {
                            title = value.to_string();
                        }
                    } else if let Token::RBrace = token {
                        tokens.next();

                        break;
                    } else {
                        tokens.next(); // Skip {

                        let node = Parser::parse_element(&mut tokens);
                        children.push(node);
                    }
                }

                ASTNode::App { children, title }
            }
            _ => panic!("Expected 'app'"),
        }
    }

    fn parse_element<'a>(tokens: &mut Peekable<impl Iterator<Item = &'a Token>>) -> ASTNode {
        if let Some(Token::Gt) = tokens.peek() {
            tokens.next();

            let name = if let Some(Token::Identifier(id)) = tokens.next() {
                id.clone()
            } else {
                panic!("Expected identifier after '>'");
            };

            let mut value = String::new();
            let mut children = Vec::new();

            let mut styles = Styles(HashMap::new());

            // Value
            if let Some(Token::Colon) = tokens.peek() {
                tokens.next();

                value = if let Some(Token::Literal(val)) = tokens.next() {
                    val.clone()
                } else {
                    panic!("Expected string literal after ':'");
                };
            }

            // Styling
            if let Some(Token::Hash) = tokens.peek() {
                tokens.next(); // skip #

                if let Some(Token::LBrace) = tokens.peek() {
                    tokens.next(); // skip {

                    while let Some(token) = tokens.peek() {
                        if matches!(token, Token::RBrace) {
                            tokens.next(); // skip }
                            break;
                        }

                        if matches!(token, Token::Comma) {
                            tokens.next(); // skip ,
                        }

                        let key = if let Some(Token::Identifier(k)) = tokens.next() {
                            k.clone()
                        } else {
                            panic!("Expected style key identifier");
                        };

                        if let Some(Token::Colon) = tokens.next() {
                        } else {
                            panic!("Expected ':' after style key");
                        }

                        let value = match tokens.next() {
                            Some(Token::Identifier(v)) => v.clone(),
                            Some(Token::Number(n)) => n.to_string().clone(),
                            other => panic!("Expected style value, found {other:?}"),
                        };

                        styles.0.insert(key, value);
                    }
                } else {
                    panic!("Expected '{{' after '#'");
                }
            }

            // Nesting
            if let Some(Token::LBrace) = tokens.peek() {
                tokens.next();

                while let Some(token) = tokens.peek() {
                    if matches!(token, Token::RBrace) {
                        tokens.next();
                        break;
                    }

                    let child = Parser::parse_element(tokens);
                    children.push(child);
                }
            }

            ASTNode::Element {
                name,
                value,
                children,
                styles,
            }
        } else {
            panic!("Expected '>' at start of element");
        }
    }
}
