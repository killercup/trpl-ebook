use std::error::Error;
use regex::Regex;

use helpers::normalize_code_blocks::*;

fn normalize_links(input: &str) -> Result<String, Box<Error>> {
    // All back-references seem to just go right back to the root
    let mut output = input.replace(r"../", r"http://doc.rust-lang.org/");
    
    let cross_section_link = Regex::new(r"]\((?P<file>[\w-_]+)\.html\)").unwrap();
    output = cross_section_link.replace_all(&output, r"](#sec--$file)");

    let cross_section_ref = Regex::new(r"(?m)^\[(?P<id>.+)\]:\s(?P<file>[^:^/]+)\.html$").unwrap();
    output = cross_section_ref.replace_all(&output, r"[$id]: #sec--$file");

    let cross_subsection_link = Regex::new(r"]\((?P<file>[\w-_]+)\.html#(?P<subsection>[\w-_]+)\)").unwrap();
    output = cross_subsection_link.replace_all(&output, r"](#$subsection)");

    let cross_subsection_ref = Regex::new(r"(?m)^\[(?P<id>.+)\]:\s(?P<file>[^:^/]+)\.html#(?P<subsection>[\w-_]+)$").unwrap();
    output = cross_subsection_ref.replace_all(&output, r"[$id]: #$subsection");

    Ok(output)
}

fn normalize_math(input: &str) -> Result<String, Box<Error>> {
    let superscript = Regex::new(r"(\d+)<sup>(\d+)</sup>").unwrap();
    Ok(superscript.replace_all(&input, r"$1^$2^"))
}


pub fn normalize(input: &str) -> Result<String, Box<Error>> {
    let mut output;

    output = try!(break_code_blocks(&input, 87, "↳ "));
    output = try!(normalize_code_start(&output));
    output = try!(normalize_links(&output));
    output = try!(normalize_math(&output));

    Ok(output)
}
