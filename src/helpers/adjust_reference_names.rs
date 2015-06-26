use std::error::Error;
use regex::{Regex, Captures};

const CODE_BLOCK_TOGGLE: &'static str = "```";

pub fn adjust_reference_name(input: &str, prefix: &str) -> Result<String, Box<Error>> {
    let reference_link = Regex::new(r"(?x)
        \]\[                # This is a link to a reference
        (?P<id>.+?)         # The reference name
        \]
    ").unwrap();

    let footnote = Regex::new(r"(?x)
        \[                  # This is a link to a reference
        \^                  # Link to footnote begins with `^`
        (?P<id>.+?)         # The reference name
        \]
    ").unwrap();

    let reference_def = Regex::new(r"(?x)
        ^
        \[
        (?P<footnote>\^)?   # Footnote definition begins with `^`
        (?P<id>.+)          # The reference name
        \]
        :\s
        (?P<link>.+)        # The link url (and maybe the title)
        $
    ").unwrap();

    let mut in_code_block = false;

    let output = input.lines()
    .fold(String::new(), |initial, line| {
        match (in_code_block, line.starts_with(CODE_BLOCK_TOGGLE)) {
            (true,  false) => {
                // Code? I don't care about that stuff.
                return initial + line + "\n";
            }
            (true,  true ) => { in_code_block = false; }
            (false, true ) => { in_code_block = true; }
            _ => {}
        };

        if reference_link.is_match(line) {
            let new_line = reference_link.replace_all(line, |matches: &Captures| {
                format!("][{prefix}--{id}]",
                    prefix = prefix,
                    id = matches.name("id").expect("no id in ref link")
                )
            });
            return initial + &new_line + "\n";
        }

        if footnote.is_match(line) {
            let new_line = footnote.replace_all(line, |matches: &Captures| {
                format!("[^{prefix}--{id}]",
                    prefix = prefix,
                    id = matches.name("id").expect("no id in ref link")
                )
            });
            return initial + &new_line + "\n";
        }

        if let Some(matches) = reference_def.captures(line) {
            let new_line = format!("[{footnote}{prefix}--{id}]: {link}",
                footnote = matches.name("footnote").unwrap_or(""),
                prefix = prefix,
                id = matches.name("id").expect("no id in ref def"),
                link = matches.name("link").expect("no ink in ref def")
            );
            return initial + &new_line + "\n";
        }

        initial + line + "\n"
    });

    Ok(output)
}

#[test]
fn reference_renamer() {
    assert_eq!(
        adjust_reference_name(
            "Lorem ipsum [dolor sit][amet], [consectetur adipisicing][elit]. Odio provident repellendus temporibus possimus magnam odit [neque obcaecati][illo], ab tenetur deserunt quae quia? Asperiores a hic, maiores quaerat, autem ea!",
            "PREFIX"
        ).unwrap(),
        "Lorem ipsum [dolor sit][PREFIX--amet], [consectetur adipisicing][PREFIX--elit]. Odio provident repellendus temporibus possimus magnam odit [neque obcaecati][PREFIX--illo], ab tenetur deserunt quae quia? Asperiores a hic, maiores quaerat, autem ea!\n"

    );
}
