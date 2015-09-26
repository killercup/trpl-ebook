use std::error::Error;
use std::path::Path;
use std::fs;
use std::collections::BTreeMap;
use std::ascii::AsciiExt;

use regex::Regex;

const FILENAME_PATTRN: &'static str =
    r"^(?P<prefix>\w+)-(?P<date>\d{4}-\d{2}-\d{2})\.(?P<ext>.+)$";

type FileListing = Vec<(String, String)>;

fn list_file_groups(path: &str) -> Result<FileListing, Box<Error>> {
    let filename_pattern = Regex::new(FILENAME_PATTRN).unwrap();

    let files = try!(fs::read_dir(&Path::new(path)))
    .filter(Result::is_ok)
    .map(|x| x.unwrap().path())
    .filter_map(|x| {
        x.file_name()
         .and_then(|a| { a.to_str() })
         .and_then(|b| -> Option<String> { Some(b.into()) })
    })
    .flat_map(|name| -> Option<(String, String)> {
        // Extract the date from names like 'trpl-2015-05-13.a4.pdf'.
        // This also excludes the `index.html` file as it contains no date.
        if let Some(caps) = filename_pattern.captures(&name) {
            if let Some(version) = caps.name("date") {
                return Some((version.to_owned(), name.to_owned()));
            }
        }
        return None;
    })
    .collect();

    Ok(files)
}

pub fn render_index(path: &str) -> Result<String, Box<Error>> {
    let filename_pattern = Regex::new(FILENAME_PATTRN).unwrap();

    let files = try!(list_file_groups(path));
    let mut versions: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for &(ref version, ref filename) in &files {
        let file = versions.entry(version.clone()).or_insert(vec![]);
        file.push(filename.clone());
    }

    let mut file_listing = String::new();

    for (version, files) in versions.iter().rev() {
        file_listing.push_str("<li>\n<h2>");
        file_listing.push_str(&version);
        file_listing.push_str("</h2>\n<ul>");

        for file in files {
            file_listing.push_str(&format!(
                "<li><a href='{file_name}'>{file_title}</a></li>\n",
                file_name = file,
                file_title = filename_pattern
                    .replace_all(file, "$prefix $ext")
                    .to_ascii_uppercase()
            ));
        }

        file_listing.push_str("</ul>\n</li>");
    }

    let output = format!(include_str!("../../lib/index_template.html"),
                         css = include_str!("../../lib/index.css"),
                         file_listing = file_listing);

    Ok(output)
}
