use std::io::prelude::*;
use std::error::Error;
use std::path::Path;
use std::fs::File;

pub fn get_file_content<P: AsRef<Path>>(path: P) -> Result<String, Box<Error>> {
    let mut file = try!(File::open(path));
    let mut buffer = String::new();
    try!(file.read_to_string(&mut buffer));
    Ok(buffer)
}

pub fn write_string_to_file(input: &str, name: &str) -> Result<(), Box<Error>> {
    let mut file = try!(File::create(&Path::new(name)));

    try!(file.write_all(input.as_bytes()));

    Ok(())
}
