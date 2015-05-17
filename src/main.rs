//! Compile EBooks for 'The Rust Programming Language'
//!
//! ['The Rust Programming Language'][trpl] is originally published as Markdown
//! and rendered by _rustbook_. This set of scripts does some transformations
//! and uses _Pandoc_ to render it as HTML, EPUB and PDF (usign LaTeX).
//!
//! [trpl]: http://doc.rust-lang.org/book/

extern crate regex;

pub mod helpers;
pub mod convert_book;

use std::error::Error;
use convert_book::options;
use convert_book::pandoc::run as pandoc;

fn save_as(book: &str, format: &str, opts: &str) -> Result<(), Box<Error>> {
    use std::ascii::AsciiExt;

    let opts = format!(
        "--from={markdown_opts} {opts} --output=dist/trpl-{release_date}.{format}",
        markdown_opts = options::MARKDOWN,
        opts = opts,
        release_date = options::RELEASE_DATE,
        format = format
    );

    try!(pandoc(&opts, &book));

    println!("[✓] {}", format.to_ascii_uppercase());

    Ok(())
}

fn render_book() -> Result<(), Box<Error>> {
    let book = try!(convert_book::markdown::to_single_file(
        "../src/SUMMARY.md",
        &format!(include_str!("book_meta.yml"), release_date = options::RELEASE_DATE)
    ));

    helpers::file::write_string_to_file(&book,
        &format!("dist/trpl-{}.md", options::RELEASE_DATE)
    ).unwrap();
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

fn main() {
    render_book().unwrap();

    let index = convert_book::index::render_index("dist/").unwrap();
    helpers::file::write_string_to_file(&index, "dist/index.html").unwrap();
    println!("[✓] {}", "Index");
}
