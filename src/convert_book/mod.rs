//! Tools to compile the book

pub mod index;
pub mod markdown;
pub mod options;
pub mod pandoc;

use std::path::Path;
use std::error::Error;
use helpers;
use convert_book::pandoc::save_as;

/// Render book in different formats
pub fn render_book(prefix: &str, src_path: &Path, meta_file: &str) -> Result<(), Box<Error>> {
    let meta_data = try!(helpers::file::get_file_content(meta_file));

    let book = try!(markdown::to_single_file(src_path,
                                             &meta_data.replace("{release_date}",
                                                                options::RELEASE_DATE)));

    try!(helpers::file::write_string_to_file(&book,
                                             &format!("dist/{}-{}.md",
                                                      prefix,
                                                      options::RELEASE_DATE)));
    println!("[✓] {}", "MD");

    try!(save_as(&book, prefix, "html", options::HTML));
    try!(save_as(&book, prefix, "epub", options::EPUB));
    try!(save_as(&book, prefix, "tex", options::LATEX));

    let plain_book = helpers::remove_emojis::remove_emojis(&book);
    try!(save_as(&plain_book,
                 prefix,
                 "a4.pdf",
                 &format!(r"{} --variable papersize=a4paper", options::LATEX)));
    try!(save_as(&plain_book,
                 prefix,
                 "letter.pdf",
                 &format!(r"{} --variable papersize=letterpaper",
                          options::LATEX)));

    Ok(())
}
