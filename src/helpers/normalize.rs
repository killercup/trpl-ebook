use std::error::Error;

use helpers::normalize_code_blocks::*;

fn normalize_links(input: &str) -> Result<String, Box<Error>> {
    let cross_section_link = regex!(r"]\(([\w-_]+)\.html\)");

    let output = input
    .replace(r"../std", r"http://doc.rust-lang.org/std")
    .replace(r"../reference", r"http://doc.rust-lang.org/reference")
    .replace(r"../rustc", r"http://doc.rust-lang.org/rustc")
    .replace(r"../syntax", r"http://doc.rust-lang.org/syntax")
    .replace(r"../core", r"http://doc.rust-lang.org/core");

    Ok(cross_section_link.replace_all(&output, r"](#sec--$1)"))
}


pub fn normalize(input: &str) -> Result<String, Box<Error>> {
    let mut output;

    output = try!(break_code_blocks(&input, 87, "↳ "));
    output = try!(normalize_code_start(&output));
    output = try!(normalize_links(&output));

    Ok(output)
}
