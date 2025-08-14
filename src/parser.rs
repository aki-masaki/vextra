use crate::ast::ASTNode;
use crate::ast::Attributes;
use crate::ast::State;
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
    Number(i16),
    LBrace,
    RBrace,
    Gt,
    Colon,
    Eof,
    Hash,
    Comma,
    At,
    Percent,
    Semicolon,
    Tilde,
    Dollar,
    Equals,
    QuestionMark,
    LBracket,
    RBracket,
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
                '@' => tokens.push(Token::At),
                '%' => tokens.push(Token::Percent),
                ';' => tokens.push(Token::Semicolon),
                '~' => tokens.push(Token::Tilde),
                '?' => tokens.push(Token::QuestionMark),
                '[' => tokens.push(Token::LBracket),
                ']' => tokens.push(Token::RBracket),
                '=' => {
                    if let Some('=') = chars.peek() {
                        tokens.push(Token::Equals);

                        chars.next();
                    }
                }
                '$' => {
                    tokens.push(Token::Dollar);

                    if let Some('$') = chars.peek() {
                        chars.next();
                    } else {
                        continue;
                    }

                    // Skip "logic `"
                    while let Some(&next) = chars.peek() {
                        chars.next();

                        if next == '`' {
                            break;
                        }
                    }

                    let mut literal = String::new();

                    while let Some(&next) = chars.peek() {
                        if next != '`' {
                            literal.push(chars.next().unwrap());
                        } else {
                            chars.next();

                            break;
                        }
                    }

                    tokens.push(Token::Literal(literal));
                }
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

                let mut state = State(HashMap::new());
                let mut logic_code = String::new();

                while let Some(token) = tokens.peek().cloned() {
                    match token {
                        Token::Colon => {
                            tokens.next();

                            if let Some(Token::Literal(value)) = tokens.next() {
                                title = value.to_string();
                            }
                        }
                        Token::RBrace => {
                            tokens.next();

                            break;
                        }
                        Token::Semicolon => {
                            tokens.next(); // skip first ;

                            while let Some(next) = tokens.peek() {
                                if matches!(next, Token::Semicolon) {
                                    tokens.next(); // skip second ;

                                    break;
                                }

                                tokens.next();
                            }
                        }
                        Token::LBrace => {
                            tokens.next();
                        }
                        Token::At => {
                            state = Parser::handle_remember(&mut tokens);
                        }
                        Token::Dollar => {
                            tokens.next(); // skip $

                            if let Some(Token::Literal(value)) = tokens.next() {
                                logic_code = value.to_string();
                            }
                        }
                        _ => {
                            let node = Parser::parse_element(&mut tokens);

                            if let Some(node) = node {
                                children.push(node);
                            } else {
                                tokens.next();
                            }
                        }
                    }
                }

                ASTNode::App {
                    children,
                    title,
                    state,
                    logic_code,
                }
            }
            _ => panic!("Expected 'app'"),
        }
    }

    fn handle_remember<'a>(tokens: &mut Peekable<impl Iterator<Item = &'a Token>>) -> State {
        let mut state = State(HashMap::new());

        tokens.next(); // skip @

        if let Some(Token::Identifier(id)) = tokens.next() {
            if id != "remember" {
                panic!("Expected 'remember' after '@'");
            }

            match tokens.next() {
                Some(Token::LBrace) => {}
                _ => panic!("Expected block after '@remember'"),
            }

            loop {
                match tokens.peek() {
                    Some(Token::RBrace) => {
                        tokens.next(); // skip }
                        break;
                    }

                    Some(_) => {
                        let key = match tokens.next() {
                            Some(Token::Identifier(k)) => k.clone(),
                            _ => panic!("Expected identifier for state key"),
                        };

                        match tokens.next() {
                            Some(Token::Colon) => {}
                            _ => panic!("Expected ':' after key"),
                        }

                        let value = match tokens.next() {
                            Some(Token::Identifier(v)) => v.clone(),
                            Some(Token::Number(n)) => n.to_string(),
                            Some(Token::Literal(v)) => v.clone(),
                            Some(Token::LBracket) => {
                                let mut content = String::from("[");

                                while let Some(token) = tokens.peek() {
                                    if matches!(token, Token::RBracket) {
                                        tokens.next();

                                        break;
                                    }

                                    match token {
                                        Token::Literal(v) => content.push_str(v),
                                        Token::Number(n) => {
                                            content.push_str(n.to_string().as_str());
                                        }
                                        _ => {
                                            panic!("Unexpected token in array");
                                        }
                                    }

                                    tokens.next();
                                }

                                content.push(']');

                                content
                            }
                            other => panic!("Expected value, got {other:?}"),
                        };

                        state.0.insert(key, value);

                        if let Some(Token::Comma) = tokens.peek() {
                            tokens.next(); // skip ,
                        }
                    }

                    None => {
                        panic!("Unexpected end of tokens in @remember block")
                    }
                }
            }
        }

        state
    }

    fn handle_input_binding<'a>(
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Attributes {
        let mut attributes = Attributes(HashMap::new());

        tokens.next();

        let binding = if let Some(Token::Identifier(val)) = tokens.next() {
            val.clone()
        } else {
            panic!("Expected identifier after '~'");
        };

        attributes.0.insert(String::from("data-model"), binding);

        attributes
    }

    fn handle_button_binding<'a>(
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Attributes {
        tokens.next(); // Skip $

        let mut attributes = Attributes(HashMap::new());

        let event = if let Some(Token::Identifier(val)) = tokens.next() {
            val.clone()
        } else {
            panic!("Expected identifier after '$'");
        };

        if let Some(Token::Colon) = tokens.next() {
        } else {
            panic!("Expected : after '$' event key");
        }

        let code = if let Some(Token::Literal(code)) = tokens.next() {
            code.clone()
        } else {
            panic!("Expected literal after ':'");
        };

        attributes.0.insert(format!("on{event}"), code);

        attributes
    }

    fn handle_style<'a>(tokens: &mut Peekable<impl Iterator<Item = &'a Token>>) -> Styles {
        tokens.next(); // skip #

        let mut styles = Styles(HashMap::new());

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

                if let Some(Token::Colon) = tokens.peek() {
                    tokens.next();
                }

                let value = match tokens.peek() {
                    Some(Token::Identifier(v)) => {
                        tokens.next();
                        v.clone()
                    }
                    Some(Token::Number(n)) => {
                        tokens.next();
                        n.to_string().clone()
                    }
                    Some(Token::Comma) => String::from("true"),
                    Some(Token::RBrace) => String::from("true"),
                    other => panic!("Expected style value, found {other:?}"),
                };

                styles.0.insert(key, value);
            }
        } else {
            panic!("Expected '{{' after '#'");
        }

        styles
    }

    fn handle_attributes<'a>(tokens: &mut Peekable<impl Iterator<Item = &'a Token>>) -> Attributes {
        tokens.next(); // skip %

        let mut attributes = Attributes(HashMap::new());

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
                    panic!("Expected attribute key identifier");
                };

                if let Some(Token::Colon) = tokens.next() {
                } else {
                    panic!("Expected ':' after attribute key");
                }

                let value = match tokens.next() {
                    Some(Token::Identifier(v)) => v.clone(),
                    Some(Token::Number(n)) => n.to_string().clone(),
                    other => panic!("Expected attribute value, found {other:?}"),
                };

                attributes.0.insert(key, value);
            }
        } else {
            panic!("Expected '{{' after '%'");
        }

        attributes
    }

    fn handle_conditional_rendering<'a>(
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Attributes {
        tokens.next(); // skip ?

        let mut attributes = Attributes(HashMap::new());

        if let Some(Token::LBrace) = tokens.peek() {
            tokens.next(); // skip {

            while let Some(token) = tokens.peek() {
                if matches!(token, Token::RBrace) {
                    tokens.next(); // skip }
                    break;
                }

                let left = if let Some(Token::Identifier(k)) = tokens.next() {
                    k.clone()
                } else {
                    panic!("Expected identifier for left operand");
                };

                if let Some(Token::Equals) = tokens.next() {
                } else {
                    panic!("Expected '=='");
                }

                let right = match tokens.next() {
                    Some(Token::Number(n)) => n.to_string().clone(),
                    Some(Token::Literal(l)) => l.clone(),
                    other => panic!("Expected attribute value, found {other:?}"),
                };

                attributes
                    .0
                    .insert(String::from("data-if"), format!("{left},{right}"));
            }
        } else {
            panic!("Expected '{{' after '?'");
        }

        attributes
    }

    fn parse_element<'a>(
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Option<ASTNode> {
        // Comments
        if let Some(Token::Semicolon) = tokens.peek() {
            tokens.next(); // skip first ;

            while let Some(next) = tokens.peek() {
                if matches!(next, Token::Semicolon) {
                    tokens.next(); // skip second ;

                    break;
                }

                tokens.next();
            }
        }

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
            let mut attributes = Attributes(HashMap::new());

            while let Some(token) = tokens.peek() {
                match token {
                    Token::Tilde => {
                        attributes.0.extend(Parser::handle_input_binding(tokens).0);
                    }
                    Token::Dollar => {
                        attributes.0.extend(Parser::handle_button_binding(tokens).0);
                    }
                    Token::Colon => {
                        tokens.next();

                        value = if let Some(Token::Literal(val)) = tokens.next() {
                            val.clone()
                        } else {
                            panic!("Expected string literal after ':'");
                        };
                    }
                    Token::Hash => {
                        styles = Parser::handle_style(tokens);
                    }
                    Token::Percent => {
                        attributes.0.extend(Parser::handle_attributes(tokens).0);
                    }
                    Token::QuestionMark => {
                        attributes
                            .0
                            .extend(Parser::handle_conditional_rendering(tokens).0);
                    }
                    Token::LBrace => break, // done with attributes/modifiers
                    _ => break,
                }
            }

            if let Some(Token::LBrace) = tokens.peek() {
                tokens.next();

                let mut depth = 1;

                while let Some(token) = tokens.peek() {
                    match token {
                        Token::LBrace => {
                            tokens.next();
                            depth += 1;
                        }
                        Token::RBrace => {
                            tokens.next();
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => {
                            let child = Parser::parse_element(tokens);

                            if let Some(child) = child {
                                children.push(child);
                            }
                        }
                    }
                }
            }

            Some(ASTNode::Element {
                name,
                value,
                children,
                styles,
                attributes,
            })
        } else if let Some(Token::RBrace) = tokens.peek() {
            None
        } else if let Some(Token::LBracket) = tokens.peek() {
            tokens.next(); // skip [

            let list = if let Some(Token::Identifier(list)) = tokens.next() {
                list.clone()
            } else {
                panic!("Expected identifier after '['")
            };

            tokens.next(); // skip ]

            let styles = if let Some(Token::Hash) = tokens.peek() {
                Parser::handle_style(tokens)
            } else {
                Styles(HashMap::new())
            };

            tokens.next(); // skip {

            let mut children = vec![];

            let node = Parser::parse_element(tokens);

            if let Some(node) = node {
                children.push(node);
            }

            tokens.next(); // skip }

            let mut attributes = Attributes(HashMap::new());

            attributes.0.insert(String::from("data-list"), list);

            Some(ASTNode::Element {
                name: String::from("div"),
                value: String::new(),
                children,
                styles,
                attributes,
            })
        } else {
            panic!("Expected '>' at start of element");
        }
    }
}
