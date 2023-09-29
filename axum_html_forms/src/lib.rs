#[derive(Debug, Deserialize, Serialize)]
pub enum FormInputType {
    Email,
    Password,
    Text,
}

impl std::fmt::Display for FormInputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormInputType::Email => write!(f, "email"),
            FormInputType::Password => write!(f, "password"),
            FormInputType::Text => write!(f, "text"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormInput {
    pub input_type: FormInputType,
    pub name: String,
    pub label: String,
    pub value: Option<String>,
    pub errors: FormErrors,
}

impl HtmlField for FormInput {
    fn render(&self) -> Node {
        FormInput::field(
            &self.errors,
            &self.name,
            &self.label,
            FormInput::input(
                &self.input_type.to_string(),
                &self.name,
                self.value.as_deref().unwrap_or(""),
                &self.errors,
            ),
        )
    }
}

pub trait HtmlField {
    fn render(&self) -> Node;

    fn field(errors: &FormErrors, name: &str, label: &str, inner: Node) -> Node {
        html! {
            <label for=format!("label_{name}")>{text!("{label}")}
            {
                if errors.is_empty() {
                    html!{<>}
                } else {
                    html! { <ul>
                        { errors.iter().map(|error| {
                            html! { <li>{text!("{error}")}</li> }
                        })}
                    </ul> }
                }
            }
            { inner }
            </label>
        }
    }
    fn input(input_type: &str, name: &str, value_str: &str, errors: &FormErrors) -> Node {
        html! {
            <input
                name=format!("{name}")
                type=format!("{input_type}")
                value=format!("{value_str}")
                id=format!("label_{name}")
                {
                    if errors.is_empty() {
                        String::new()
                    } else {
                        String::from("aria-invalid=true")
                    }
                }
            />
        }
    }
}

pub type FormErrors = Vec<String>;

use html_node::html;
use html_node::text;
use html_node::Node;
use serde::Deserialize;
use serde::Serialize;
