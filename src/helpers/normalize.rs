use std::error::Error;

use helpers::normalize_code_blocks::*;

fn normalize_links(input: &str) -> Result<String, Box<Error>> {
    let mut output = input
    .replace(r"../std", r"http://doc.rust-lang.org/std")
    .replace(r"../reference", r"http://doc.rust-lang.org/reference")
    .replace(r"../rustc", r"http://doc.rust-lang.org/rustc")
    .replace(r"../syntax", r"http://doc.rust-lang.org/syntax")
    .replace(r"../core", r"http://doc.rust-lang.org/core");

    let cross_section_link = regex!(r"]\((?P<file>[\w-_]+)\.html\)");
    output = cross_section_link.replace_all(&output, r"](#sec--$file)");

    let cross_section_ref = regex!(r"(?m)^\[(?P<id>.+)\]:\s(?P<file>[^:^/]+)\.html$");
    output = cross_section_ref.replace_all(&output, r"[$id]: #sec--$file");

    let cross_subsection_link = regex!(r"]\((?P<file>[\w-_]+)\.html#(?P<subsection>[\w-_]+)\)");
    output = cross_subsection_link.replace_all(&output, r"](#$subsection)");

    let cross_subsection_ref = regex!(r"(?m)^\[(?P<id>.+)\]:\s(?P<file>[^:^/]+)\.html#(?P<subsection>[\w-_]+)$");
    output = cross_subsection_ref.replace_all(&output, r"[$id]: #$subsection");

    Ok(output)
}

fn normalize_math(input: &str) -> Result<String, Box<Error>> {
    let superscript = regex!(r"(\d+)<sup>(\d+)</sup>");
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
