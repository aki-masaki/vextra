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
    },
}

impl ASTNode {
    pub fn render_html(&self) -> String {
        match self {
            ASTNode::App { children, title } => {
                let mut html = format!("<html><head><title>{title}</title></head><body>");

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
            } => {
                let mut html = String::new();
                let tag_name = ASTNode::get_tag_name(name.to_string());

                html.push_str(format!("<{}>", tag_name).as_str());

                html.push_str(value);

                for child in children {
                    html.push_str(&child.render_html());
                }

                html.push_str(format!("</{}>", tag_name).as_str());

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
}
