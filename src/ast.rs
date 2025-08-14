use std::collections::HashMap;

#[derive(Debug)]
pub struct Styles(pub HashMap<String, String>);

#[derive(Debug)]
pub struct Attributes(pub HashMap<String, String>);

#[derive(Debug)]
pub struct State(pub HashMap<String, String>);

#[derive(Debug)]
pub enum ASTNode {
    App {
        children: Vec<ASTNode>,
        title: String,
        state: State,
        logic_code: String,
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
    @import url('https://fonts.googleapis.com/css2?family=Source+Code+Pro:ital,wght@0,200..900;1,200..900&display=swap');

    * {
        box-sizing: border-box;
        margin: 0;
        padding: 0;
        font-family: "Source Code Pro", monospace;
    }

    body {
        background-color: black;
        color: white;
        width: 100vw;
        height: 100vh;
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

    button {
        cursor: pointer;
    }

    ::placeholder {
        color: #888;
    }
"#;

impl ASTNode {
    pub fn render_html(&self) -> String {
        match self {
            ASTNode::App {
                children,
                title,
                state,
                logic_code,
            } => {
                let mut html = format!(
                    "<!DOCTYPE html><html><head><title>{title}</title><style>{DEFAULT_STYLES}</style><script>{logic_code}</script>{}</head><body>",
                    ASTNode::render_javascript(state)
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

                if let Some(var) = ASTNode::extract_braced_var(value) {
                    html.push_str(format!("data-bind=\"{var}\"").as_str());
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

    fn extract_braced_var(s: &str) -> Option<&str> {
        let start = s.find('{')?;
        let end = s[start..].find('}')?;
        Some(&s[start + 1..start + end])
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
            "justify" => "justify-content",
            "align" => "align-items",
            "direction" => "flex-direction",
            _ => key.as_str(),
        })
    }

    pub fn convert_style_values(value: String) -> String {
        String::from(match value.as_str() {
            "red" => "#E43636",
            "green" => "#8ABB6C",
            "big" => "30px",
            "fill" => "100%",
            "vertical" => "column",
            _ => value.as_str(),
        })
    }

    pub fn render_javascript(state: &State) -> String {
        let js_state: String = state
            .0
            .iter()
            .map(|(k, v)| format!("{k}: \"{v}\""))
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            r#"
<script>
  let state = new Proxy({{ {js_state} }}, {{
    set(obj, prop, value) {{
      obj[prop] = value;
      document.querySelectorAll(`[data-bind="${{prop}}"]`).forEach(el => {{
        el.textContent = el.getAttribute("data-template").replace(/\{{(.+?)\}}/g, (_, k) => state[k]);
      }});
      document.querySelectorAll(`[data-model="${{prop}}"]`).forEach(el => {{
        el.value = value;
      }});
      return true;
    }}
  }});

  window.addEventListener('DOMContentLoaded', () => {{
    document.querySelectorAll("[data-bind]").forEach(el => {{
      el.setAttribute("data-template", el.textContent);
      el.textContent = el.textContent.replace(/\{{(.+?)\}}/g, (_, k) => state[k]);
    }});

    document.querySelectorAll('[data-model]').forEach(el => {{
      const key = el.getAttribute('data-model');
      el.addEventListener('input', () => {{
        state[key] = el.value;
      }});
    }});
  }});
</script>
"#
        )
    }
}
