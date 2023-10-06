use inner::SimpleForm;

mod inner {
    #[derive(Debug, HtmlForm)]
    pub struct SimpleForm {
        pub name: String,
        pub age: u8,
        pub favourite_word: Option<String>,
    }

    use axum_html_forms_derive::HtmlForm;
}

fn main() {
    let original = SimpleForm {
        name: String::from("John"),
        age: 69,
        favourite_word: Some(String::from("Buns")),
    };

    let unchecked_1 = inner::SimpleFormUnchecked {
        name: Some(String::from("Jane")),
        age: Some(String::from("42")),
        favourite_word: None,
    };
    let unchecked_2 = inner::SimpleFormUnchecked {
        name: Some(String::from("Alice")),
        age: Some(String::from("69")),
        favourite_word: Some(String::from("Beans")),
    };

    let form = inner::SimpleFormHtmlForm::default();

    let nodes = form.render();
    let html = format!("{nodes:#}");
    eprintln!("{}", html);

    let parsed_form = SimpleForm::try_from(&unchecked_1);

    let _ = original.name;
    let _ = original.age;
    let _ = unchecked_1;
    let _ = unchecked_2;
    let _ = form;
    let _ = parsed_form;
}
