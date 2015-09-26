//! Compile EBooks for 'The Rust Programming Language'
//!
//! ['The Rust Programming Language'][trpl] is originally published as Markdown
//! and rendered by _rustbook_. This set of scripts does some transformations
//! and uses _Pandoc_ to render it as HTML, EPUB and PDF (usign LaTeX).
//!
//! [trpl]: http://doc.rust-lang.org/book/

#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate regex;
extern crate docopt;
extern crate rustc_serialize;

use std::path::Path;
use docopt::Docopt;

pub mod helpers;
pub mod convert_book;

static USAGE: &'static str = r#"
Compile Rustbook to EBook formats.

Usage:
  compile-trpl [--prefix=<prefix>] [--source=<directory>] [--meta=<meta_file>]

Options:
  --prefix=<prefix>     Prefix/short name of your book, e.g. "trpl" or "nomicon".
  --source=<directory>  Directory containing the git book files, especially SUMMARY.md and README.md.
  --meta=<meta_file>    Meta data of your book, needs to contain `date: {release_date}`.
"#;

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_prefix: Option<String>,
    flag_source: Option<String>,
    flag_meta: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let prefix = args.flag_prefix.unwrap_or("trpl".to_owned());
    let source = args.flag_source.unwrap_or("trpl".to_owned());
    let meta = args.flag_meta.unwrap_or("trpl_meta.yml".to_owned());

    convert_book::render_book(&prefix, &Path::new(&source), &meta).unwrap();

    let index = convert_book::index::render_index("dist/").unwrap();
    helpers::file::write_string_to_file(&index, "dist/index.html").unwrap();
    println!("[✓] {}", "Index");
}
