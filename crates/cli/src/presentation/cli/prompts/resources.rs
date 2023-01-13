use inquire::{error::InquireResult, Text};

use crate::domain::entities::resource::BookBuilder;

pub fn add_book() -> InquireResult<BookBuilder> {
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

    let mut book_builder = BookBuilder::new();
    book_builder
        .with_title(title)
        .with_author(author)
        .with_authors(authors);

    Ok(book_builder)
}
