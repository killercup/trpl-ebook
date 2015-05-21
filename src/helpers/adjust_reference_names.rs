use std::error::Error;
use regex::Captures;

const CODE_BLOCK_TOGGLE: &'static str = "```";

pub fn adjust_reference_name(input: &str, prefix: &str) -> Result<String, Box<Error>> {
    let reference_link = regex!(r"\[(?P<title>.+)\]\[(?P<id>.+)\]");
    let reference_def = regex!(r"^\[(?P<id>.+)\]:\s(?P<link>.+)$");

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
                format!("[{title}][{prefix}--{id}]",
                    title = matches.name("title").expect("no title in ref link"),
                    prefix = prefix,
                    id = matches.name("id").expect("no id in ref link")
                )
            });
            return initial + &new_line + "\n";
        }

        if let Some(def) = reference_def.captures(line) {
            let new_line = format!("[{prefix}--{id}]: {link}",
                prefix = prefix,
                id = def.name("id").expect("no id in ref def"),
                link = def.name("link").expect("no ink in ref def")
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
            "Lorem ipsum dolor [sit][amet], consectetur adipisicing elit. Odio provident repellendus temporibus possimus magnam odit neque obcaecati illo, ab tenetur deserunt quae quia? Asperiores a hic, maiores quaerat, autem ea!",
            "DANGER"
        ).unwrap(),
        "Lorem ipsum dolor [sit][DANGER--amet], consectetur adipisicing elit. Odio provident repellendus temporibus possimus magnam odit neque obcaecati illo, ab tenetur deserunt quae quia? Asperiores a hic, maiores quaerat, autem ea!\n"

    );
}
