//! Compile EBooks for 'The Rust Programming Language'
//!
//! ['The Rust Programming Language'][trpl] is originally published as Markdown
//! and rendered by _rustbook_. This set of scripts does some transformations
//! and uses _Pandoc_ to render it as HTML, EPUB and PDF (usign LaTeX).
//!
//! [trpl]: http://doc.rust-lang.org/book/

#![feature(plugin)]
#![plugin(regex_macros)]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate regex;

pub mod helpers;
pub mod convert_book;

fn main() {
    convert_book::render_book().unwrap();

    let index = convert_book::index::render_index("dist/").unwrap();
    helpers::file::write_string_to_file(&index, "dist/index.html").unwrap();
    println!("[✓] {}", "Index");
}
