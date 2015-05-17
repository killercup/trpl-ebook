//! Compile EBooks for 'The Rust Programming Language'
//!
//! ['The Rust Programming Language'][trpl] is originally published as Markdown
//! and rendered by _rustbook_. This set of scripts does some transformations
//! and uses _Pandoc_ to render it as HTML, EPUB and PDF (usign LaTeX).
//!
//! [trpl]: http://doc.rust-lang.org/book/

extern crate regex;
extern crate itertools;

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

fn main() {
    let book = convert_book::markdown::to_single_file(
        "../src/SUMMARY.md",
        &format!(include_str!("book_meta.yml"), release_date = options::RELEASE_DATE)
    ).unwrap();

    helpers::file::write_string_to_file(&book, "dist/_all.md").unwrap();
    println!("[✓] {}", "MD");

    save_as(&book, "html", options::HTML).unwrap();
    save_as(&book, "epub", options::EPUB).unwrap();
    save_as(&book, "tex", options::LATEX).unwrap();

    let plain_book = helpers::remove_emojis::remove_emojis(&book);
    save_as(&plain_book, "a4.pdf",
        &format!(r"{} --variable papersize=a4paper", options::LATEX)
    ).unwrap();
    save_as(&plain_book, "letter.pdf",
        &format!(r"{} --variable papersize=letterpaper", options::LATEX)
    ).unwrap();
}
