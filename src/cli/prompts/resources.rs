use inquire::{error::InquireResult, Text};

use crate::entities::resource::{BookBuilder, Kind};

pub fn add_book() -> InquireResult<Kind> {
    let title = Text::new("Title").prompt()?;
    let author = Text::new("Author").prompt()?;
    let mut authors = Vec::new();

    loop {
        let author = Text::new("Author")
            .with_help_message("Leave blank if you don't want to add more authors :3")
            .prompt()?;
        if author.is_empty() {
            break;
        }
        authors.push(author);
    }

    let book = BookBuilder::default()
        .with_title(title)
        .with_author(author)
        .with_authors(authors)
        .build();

    Ok(book)
}
