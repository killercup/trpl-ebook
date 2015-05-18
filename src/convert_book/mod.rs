//! Tools to compile the book

pub mod index;
pub mod markdown;
pub mod options;
pub mod pandoc;

use std::error::Error;
use helpers;
use convert_book::pandoc::save_as;

/// Render book in different formats
pub fn render_book() -> Result<(), Box<Error>> {
    let book = try!(markdown::to_single_file(
        "../src/SUMMARY.md",
        &format!(include_str!("../book_meta.yml"), release_date = options::RELEASE_DATE)
    ));

    try!(helpers::file::write_string_to_file(&book,
        &format!("dist/trpl-{}.md", options::RELEASE_DATE)
    ));
    println!("[✓] {}", "MD");

    try!(save_as(&book, "html", options::HTML));
    try!(save_as(&book, "epub", options::EPUB));
    try!(save_as(&book, "tex", options::LATEX));

    let plain_book = helpers::remove_emojis::remove_emojis(&book);
    try!(save_as(&plain_book, "a4.pdf",
        &format!(r"{} --variable papersize=a4paper", options::LATEX)
    ));
    try!(save_as(&plain_book, "letter.pdf",
        &format!(r"{} --variable papersize=letterpaper", options::LATEX)
    ));

    Ok(())
}
