#[derive(Debug)]
pub enum ASTNode {
    App { children: Vec<ASTNode> },
    Element { name: String, value: String },
}

impl ASTNode {
    pub fn render_html(&self) -> String {
        match self {
            ASTNode::App { children } => {
                let mut html = String::from("<html><head></head><body>");

                for child in children {
                    html.push_str(&child.render_html());
                }

                html.push_str("</body></html>");

                html
            }
            ASTNode::Element { name, value } => match name.as_str() {
                "label" => {
                    format!("<span>{value}</span>")
                }
                _ => format!("<{name}>{value}</{name}>"),
            },
        }
    }
}
