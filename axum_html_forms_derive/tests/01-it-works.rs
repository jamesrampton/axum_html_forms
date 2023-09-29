mod inner {
    #[derive(Debug, HtmlForm)]
    pub struct SimpleForm {
        name: String,
        age: u8,
    }

    use axum_html_forms_derive::HtmlForm;
}

fn main() {
    let unchecked = inner::SimpleFormUnchecked {
        name: Some(String::from("Jane")),
        age: Some(42),
    };

    // let _ = original.name;
    // let _ = original.age;
    let _ = unchecked;
}
