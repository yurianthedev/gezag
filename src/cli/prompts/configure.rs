use inquire::{error::InquireResult, Select, Text};

pub fn choose_librarian(librarians: Vec<String>) -> InquireResult<String> {
    Select::new("What libarian do you want to use?", librarians)
        .with_starting_cursor(0) // Uses the first as the default one.
        .prompt()
}

pub fn registry_location(current: Option<String>) -> InquireResult<String> {
    let mut text = Text::new("Where do you want to store your registry?");
    let default: String;

    if let Some(crt) = current {
        default = crt;
        text = text.with_default(default.as_str());
    }

    text.prompt()
}
