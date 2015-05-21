use std::error::Error;
use std::path::Path;
use std::fs;
use std::collections::BTreeMap;
use std::ascii::AsciiExt;

type FileListing = Vec<(String, String)>;

fn list_file_groups(path: &str) -> Result<FileListing, Box<Error>> {
    let files = try!(fs::read_dir(&Path::new(path)))
    .filter(Result::is_ok)
    .map(|x| x.unwrap().path())
    .filter_map(|x| {
        x.file_name()
         .and_then(|a| { a.to_str() })
         .and_then(|b| -> Option<String> { Some(b.into()) })
    })
    .filter(|x| x.starts_with("trpl"))
    .map(|name| {
        // Files are name like 'trpl-2015-05-13.a4.pdf'. The first 15 chars
        // define the release version, the first 5 are always `trpl-`, though.
        let version = name.chars().skip(5).take(10).collect::<String>();
        (version, name)
    })
    .collect();

    Ok(files)
}

pub fn render_index(path: &str) -> Result<String, Box<Error>> {
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
                file_title = file.chars().skip(15).collect::<String>()
                                 .to_ascii_uppercase()
                                 .replace("-", "").replace(".", " ")
                                 .trim()
            ));
        }

        file_listing.push_str("</ul>\n</li>");
    }

    let output = format!(
        include_str!("../../lib/index_template.html"),
        file_listing = file_listing
    );

    Ok(output)
}
