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
pub fn render_book(prefix: Option<String>, src_path: &Path, meta_file: Option<String>) -> Result<(), Box<Error>> {
    let src_folder = src_path.file_name().unwrap().to_str().unwrap();
    let new_prefix = prefix.unwrap_or(src_folder.to_string());

    let src_path_str = src_path.to_str().unwrap();

    let meta_file_path = src_path.join("meta.yml");
    let meta_file_str = meta_file_path.to_str().unwrap();
    let new_meta_file = meta_file.unwrap_or(meta_file_str.to_string());
    let meta_data = try!(helpers::file::get_file_content(new_meta_file));

    let book = try!(markdown::to_single_file(src_path,
                                             &meta_data.replace("{release_date}",
                                                                options::RELEASE_DATE)));

    try!(helpers::file::write_string_to_file(&book,
                                             &format!("dist/{}-{}.md",
                                                      &new_prefix,
                                                      options::RELEASE_DATE)));
    println!("[✓] {}", "MD");

    try!(save_as(&book, &new_prefix, "html", options::HTML, src_path_str));
    try!(save_as(&book, &new_prefix, "epub", options::EPUB, src_path_str));

    let cc_book = helpers::convert_checkmarks::convert_checkmarks(&book);    
    try!(save_as(&cc_book, &new_prefix, "tex", options::LATEX, src_path_str));

    let plain_book = helpers::remove_emojis::remove_emojis(&cc_book);
    try!(save_as(&plain_book,
                 &new_prefix,
                 "a4.pdf",
                 &format!(r"{} --variable papersize=a4paper", options::LATEX),
                 src_path_str));

    try!(save_as(&plain_book,
                 &new_prefix,
                 "letter.pdf",
                 &format!(r"{} --variable papersize=letterpaper", options::LATEX),
                 src_path_str));

    Ok(())
}
