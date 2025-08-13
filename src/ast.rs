use std::collections::HashMap;

#[derive(Debug)]
pub struct Styles(pub HashMap<String, String>);

#[derive(Debug)]
pub struct Attributes(pub HashMap<String, String>);

#[derive(Debug)]
pub enum ASTNode {
    App {
        children: Vec<ASTNode>,
        title: String,
    },
    Element {
        name: String,
        value: String,
        children: Vec<ASTNode>,
        styles: Styles,
        attributes: Attributes,
    },
}

const DEFAULT_STYLES: &str = r#"
    <style>
        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }

        body {
            background-color: black;
            color: white;
            font-family: monospace;
        }

        label {
            display: block;
            margin-bottom: 0.5em;
        }

        input, button, textarea {
            background: #111;
            color: white;
            border: 1px solid #333;
            padding: 0.4em;
            border-radius: 4px;
            font-size: 1.1em;
        }

        ::placeholder {
            color: #888;
        }
    </style>
"#;

impl ASTNode {
    pub fn render_html(&self) -> String {
        match self {
            ASTNode::App { children, title } => {
                let mut html = format!(
                    "<html><head><title>{title}</title><style>{DEFAULT_STYLES}</style></head><body>"
                );

                for child in children {
                    html.push_str(&child.render_html());
                }

                html.push_str("</body></html>");

                html
            }
            ASTNode::Element {
                name,
                value,
                children,
                styles,
                attributes,
            } => {
                let mut html = String::new();
                let tag_name = ASTNode::get_tag_name(name.to_string());

                if styles.0.is_empty() {
                    html.push_str(format!("<{tag_name} ").as_str());
                } else {
                    html.push_str(
                        format!("<{tag_name} style=\"{}\"", ASTNode::render_styles(styles))
                            .as_str(),
                    );
                }

                for attribute in &attributes.0 {
                    html.push_str(format!("{}=\"{}\"", attribute.0, attribute.1).as_str());
                }

                html.push('>');

                html.push_str(value);

                for child in children {
                    html.push_str(&child.render_html());
                }

                html.push_str(format!("</{tag_name}>").as_str());

                html
            }
        }
    }

    pub fn get_tag_name(name: String) -> String {
        String::from(match name.as_str() {
            "label" => "span",
            _ => name.as_str(),
        })
    }

    pub fn render_styles(styles: &Styles) -> String {
        let mut css = String::new();

        for style in styles.0.clone() {
            css.push_str(
                format!(
                    "{}:{};",
                    ASTNode::convert_style_keys(style.0),
                    ASTNode::convert_style_values(style.1)
                )
                .as_str(),
            );
        }

        css
    }

    pub fn convert_style_keys(key: String) -> String {
        String::from(match key.as_str() {
            "size" => "font-size",
            "fg" => "color",
            _ => key.as_str(),
        })
    }

    pub fn convert_style_values(value: String) -> String {
        String::from(match value.as_str() {
            "red" => "#E43636",
            "green" => "#8ABB6C",
            "big" => "30px",
            _ => value.as_str(),
        })
    }
}
