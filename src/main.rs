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
pub mod normalize_code_blocks;
pub mod remove_emojis;
pub mod normalize;
pub mod shell_pipe;
pub mod pandoc;
pub mod read_toc;
pub mod file;

const MARKDOWN_OPTIONS: &'static str = "markdown+grid_tables+pipe_tables+raw_html+implicit_figures+footnotes+intraword_underscores+auto_identifiers-inline_code_attributes";

fn main() {
    let toc = file::get_file_content("../src/SUMMARY.md").unwrap();

    let mut book = String::new();

    book.push_str(include_str!("book_meta.yml"));
    book.push_str("\n");

    let pandoc_options = format!(
        "--from={markdown_options} --to={markdown_options} --base-header-level={header_level} --indented-code-classes=rust --atx-headers",
        markdown_options = MARKDOWN_OPTIONS, header_level = 3
    );

    for chapter in &read_toc::get_chapters(&toc) {
        let path = format!("../src/{}", &chapter.file);
        let file = file::get_file_content(&path)
            .ok().expect(&format!("Couldn't read {}", &path));

        print!("{file}: Read ok.", file = &path);

        let mut content = pandoc::run(&pandoc_options, &file)
            .map_err(|err| format!("pandoc error: {}", err.description()))
            .unwrap();

        content = normalize::normalize(&content)
            .map_err(|err| format!("normalize error: {}", err.description()))
            .unwrap();

        book.push_str("\n");
        book.push_str(&chapter.headline);
        book.push_str("\n");
        book.push_str(&content);

        println!(" Processing ok. {} bytes added.", &content.len());
    }

    file::write_string_to_file(&book, "dist/_all.md").unwrap();

    println!("Wrote {} bytes.", book.len());
}
