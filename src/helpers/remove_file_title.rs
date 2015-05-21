use std::error::Error;

pub fn remove_file_title(input: &str) -> Result<String, Box<Error>> {
    Ok(regex!(r"^%\s(.+)\n").replace(input, ""))
}
