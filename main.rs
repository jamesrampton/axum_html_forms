#[derive(Debug, HtmlForm)]
pub struct SimpleForm {
    pub name: String,
    pub age: u8,
    pub favourite_word: Option<String>,
}

fn main() {
    let original = SimpleForm {
        name: String::from("John"),
        age: 69,
        favourite_word: Some(String::from("Buns")),
    };

    let unchecked = SimpleFormUnchecked {
        name: Some(String::from("Jane")),
        age: Some(String::from("42")),
        favourite_word: None,
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
