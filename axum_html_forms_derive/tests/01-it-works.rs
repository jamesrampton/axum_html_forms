use inner::SimpleForm;

mod inner {
    #[derive(Debug, HtmlForm)]
    pub struct SimpleForm {
        pub name: String,
        pub age: u8,
    }

    use axum_html_forms_derive::HtmlForm;
}

fn main() {
    let original = SimpleForm {
        name: String::from("John"),
        age: 69,
    };

    let unchecked = inner::SimpleFormUnchecked {
        name: Some(String::from("Jane")),
        age: Some(42),
    };

    let _ = original.name;
    let _ = original.age;
    let _ = unchecked;
}
