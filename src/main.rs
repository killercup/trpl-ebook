//! Compile EBooks for 'The Rust Programming Language'
//!
//! ['The Rust Programming Language'][trpl] is originally published as Markdown
//! and rendered by _rustbook_. This set of scripts does some transformations
//! and uses _Pandoc_ to render it as HTML, EPUB and PDF (usign LaTeX).
//!
//! [trpl]: http://doc.rust-lang.org/book/

extern crate regex;
extern crate itertools;

pub mod line_breaks;
pub mod code_line_breaks;
pub mod remove_emojis;
pub mod normalize;
pub mod shell_pipe;
pub mod pandoc;
pub mod read_toc;
pub mod file;

fn main() {
    let toc = file::get_file_content("../src/SUMMARY.md").unwrap();

    let mut book = String::new();

    for chapter in &read_toc::get_chapters(&toc) {
        book.push_str(&chapter.headline);
    }

    file::write_string_to_file(&book, "_all.md").unwrap();

    println!("Wrote {} lines.", book.lines().count());
}
