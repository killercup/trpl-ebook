use regex::Regex;
use std::error::Error;
use std::path::Path;

use helpers::*;

/// Poor man's progress indicator
macro_rules! put {
    ($e:expr) => {{
        {
            use std::io;
            use std::io::Write;
            print!($e);
            io::stdout().flush().unwrap();
        }
    }};
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Chapter {
    file: String,
    headline: String,
}

fn get_chapters(toc: &str) -> Vec<Chapter> {
    let toc_pattern = Regex::new(
        r"(?x)
        (?P<indent>\s*?)
        \*\s
        \[
        (?P<title>.+?)
        \]
        \(
        (?P<filename>.+?)
        \)
    ",
    ).unwrap();

    let filename_pattern = Regex::new(
        r"(?x)
        ^
        (?P<path>(.*)/)?
        (?P<name>(.*?))
        (?P<ext>\.(\w*))?
        $
    ",
    ).unwrap();

    toc.lines()
        .filter_map(|l| toc_pattern.captures(l))
        .map(|link| {
            let level = if link.name("indent").unwrap().chars().count() == 0 {
                "#"
            } else {
                "##"
            };
            let id = filename_pattern
                .captures(link.name("filename").unwrap())
                .unwrap()
                .name("name")
                .unwrap();

            let headline = format!(
                "{level} {name} {{#sec--{link}}}\n",
                level = level,
                name = link.name("title").unwrap(),
                link = id
            );

            Chapter {
                file: link.name("filename").unwrap().into(),
                headline: headline,
            }
        })
        .collect::<Vec<Chapter>>()
}

pub fn to_single_file(src_path: &Path, meta: &str) -> Result<String, Box<Error>> {
    put!("Reading book");

    let toc = try!(file::get_file_content(&src_path.join("SUMMARY.md")));
    put!(".");

    let mut book = String::new();

    book.push_str(meta);
    book.push_str("\n");

    {
        // Readme ~ "Getting Started"
        if src_path.join("README.md").exists(){
            let file = try!(file::get_file_content(&src_path.join("README.md")));
            let mut content = try!(adjust_header_level::adjust_header_level(&file, 1));
            content = try!(remove_file_title::remove_file_title(&content));
            content = try!(adjust_reference_names::adjust_reference_name(
                &content, "readme"
            ));
            content = try!(normalize::normalize(&content));

            put!(".");

            book.push_str("\n\n");
            book.push_str("# Introduction");
            book.push_str("\n\n");
        book.push_str(&content);
        }
    }

    for chapter in &get_chapters(&toc) {
        let file = try!(file::get_file_content(&src_path.join(&chapter.file)));

        let mut content = try!(adjust_header_level::adjust_header_level(&file, 3));
        content = try!(remove_file_title::remove_file_title(&content));
        content = try!(adjust_reference_names::adjust_reference_name(
            &content,
            &chapter.file
        ));
        content = try!(normalize::normalize(&content));

        put!(".");
        book.push_str("\n\n");
        book.push_str(&chapter.headline);
        book.push_str("\n");
        book.push_str(&content);
    }

    put!(" done.\n");

    Ok(book)
}
