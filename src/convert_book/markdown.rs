use std::error::Error;
use regex::Regex;

use helpers::*;
use convert_book::*;

/// Poor man's progress indicator
macro_rules! put {
    ($e:expr) => ({
        {
            use std::io;
            use std::io::Write;
            print!($e);
            io::stdout().flush().unwrap();
        }
    })
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Chapter {
    pub file: String,
    pub headline: String,
}

fn get_chapters(toc: &str) -> Vec<Chapter> {
    let toc_pattern = Regex::new(
        r"(?P<indent>\s*?)\* \[(?P<title>.+?)\]\((?P<filename>.+?)\)"
    ).unwrap();

    let filename_pattern = Regex::new(
        r"^(?P<path>(.*)/)?(?P<name>(.*?))(?P<ext>\.(\w*))?$"
    ).unwrap();

    toc.lines()
    .filter_map(|l| toc_pattern.captures(l))
    .map(|link| {
        let level = if link.name("indent").unwrap().chars().count() == 0 { "#" } else { "##" };
        let id = filename_pattern.captures(
            link.name("filename").unwrap()
        ).unwrap().name("name").unwrap();

        let headline = format!(
            "{level} {name} {{#sec--{link}}}\n",
            level = level, name = link.name("title").unwrap(), link = id
        );

        Chapter {
            file: link.name("filename").unwrap().into(),
            headline: headline,
        }
    })
    .collect::<Vec<Chapter>>()
}

pub fn to_single_file(toc_path: &str, meta: &str) -> Result<String, Box<Error>> {
    put!("Reading book: ");

    let toc = try!(file::get_file_content(toc_path));
    put!(".");

    let mut book = String::new();

    book.push_str(meta);
    book.push_str("\n");

    {
        // Readme ~ "Getting Started"
        let file = try!(file::get_file_content("../src/README.md"));
        let pandoc_options = format!(
            "--from={markdown_options} --to={markdown_options} --base-header-level={header_level} --indented-code-classes=rust --atx-headers",
            markdown_options = options::MARKDOWN, header_level = 1
        );
        let mut content = try!(pandoc::run(&pandoc_options, &file));
        content = try!(normalize::normalize(&content));

        put!(".");

        book.push_str("\n\n");
        book.push_str("# Introduction");
        book.push_str("\n\n");
        book.push_str(&content);
    }

    let pandoc_options = format!(
        "--from={markdown_options} --to={markdown_options} --base-header-level={header_level} --indented-code-classes=rust --atx-headers",
        markdown_options = options::MARKDOWN, header_level = 3
    );

    for chapter in &get_chapters(&toc) {
        let path = format!("../src/{}", &chapter.file);
        let file = try!(file::get_file_content(&path));

        let mut content = try!(pandoc::run(&pandoc_options, &file));
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
