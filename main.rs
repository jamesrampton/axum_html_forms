#[derive(Debug, HtmlForm)]
pub struct SimpleForm {
    pub name: String,
    pub age: u8,
}

fn main() {
    let original = SimpleForm {
        name: String::from("John"),
        age: 69,
    };

    let unchecked = SimpleFormUnchecked {
        name: Some(String::from("Jane")),
        age: Some(String::from("42")),
    };

    // let form_input = SimpleFormFields {
    //     name: Some(String::from("Jane")),
    //     age: Some(String::from("42")),
    // };

    let _ = original.name;
    let _ = original.age;
    let _ = unchecked;
}

use axum_html_forms_derive::HtmlForm;
