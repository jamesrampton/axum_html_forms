use inner::SimpleForm;

mod inner {
    #[derive(Debug, HtmlForm)]
    pub struct SimpleForm {
        pub name: String,
        pub age: u8,
        pub favourite_word: Option<String>,
        pub favourite_number: Option<u8>,
    }

    use axum_html_forms_derive::HtmlForm;
}

fn main() {
    let original = SimpleForm {
        name: String::from("John"),
        age: 69,
        favourite_word: Some(String::from("Buns")),
        favourite_number: Some(7),
    };

    println!("{}", original.favourite_word.unwrap());

    let unchecked_1 = inner::SimpleFormUnchecked {
        name: Some(String::from("Jane")),
        age: Some(String::from("42")),
        favourite_word: None,
        favourite_number: Some(String::from("1")),
    };
    let unchecked_2 = inner::SimpleFormUnchecked {
        name: Some(String::from("Alice")),
        age: Some(String::from("69")),
        favourite_word: Some(String::from("Beans")),
        favourite_number: Some(String::from("satre")),
    };

    let form = inner::SimpleFormHtmlForm::default();

    match SimpleForm::try_from(&unchecked_2) {
        Ok(_) => {
            eprintln!("form parsed ok");
        }
        Err(form) => {
            let nodes = form.render();
            let html = format!("{nodes:#}");
            eprintln!("{}", html);
        }
    }

    let _ = original.name;
    let _ = original.age;
    let _ = original.favourite_word;
    let _ = original.favourite_number;
    let _ = unchecked_1;
    let _ = unchecked_2;
    let _ = form;
}
