use std::error::Error;
use regex::Regex;

pub use code_line_breaks;

pub fn normalize_links(input: &str) -> Result<String, Box<Error>> {
    let cross_section_link = Regex::new(r"\]\(([\w\-\_]+)\.html\)").unwrap();

    let output = input
    .replace("../std", "http://doc.rust-lang.org/std")
    .replace("../reference", "http://doc.rust-lang.org/reference")
    .replace("../rustc", "http://doc.rust-lang.org/rustc")
    .replace("../syntax", "http://doc.rust-lang.org/syntax")
    .replace("../core", "http://doc.rust-lang.org/core");

    Ok(cross_section_link.replace_all(&output, "](#sec--$1)"))
}


pub fn normalize(input: &str) -> Result<String, Box<Error>> {
    let mut output = try!(code_line_breaks::break_code_blocks(&input, 87, "↳ "));
    output = try!(normalize_links(&output));
    Ok(output)
}
