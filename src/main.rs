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

fn main() {
    let book = convert_book::markdown::to_single_file(
        "../src/SUMMARY.md",
        include_str!("book_meta.yml")
    ).unwrap();

    helpers::file::write_string_to_file(&book, "dist/_all.md").unwrap();
    println!("[✓] Markdown ({} bytes)", &book.len());
}
