use std::error::Error;
use std::path::Path;
use std::io::Read;
use std::fs::File;

pub fn get_file_content(name: &str) -> Result<String, Box<Error>> {
    let mut file = try!(File::open(&Path::new(name)));
    let mut buffer = String::new();
    try!(file.read_to_string(&mut buffer));
    Ok(buffer)
}
