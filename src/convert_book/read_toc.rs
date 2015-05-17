use regex::Regex;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Chapter {
    pub file: String,
    pub headline: String,
}

pub fn get_chapters(toc: &str) -> Vec<Chapter> {
    let toc_pattern = Regex::new(r"(?P<indent>\s*?)\* \[(?P<title>.+?)\]\((?P<filename>.+?)\)")
    .unwrap();
    let filename_pattern = Regex::new(r"^(?P<path>(.*)/)?(?P<name>(.*?))(?P<ext>\.(\w*))?$").unwrap();

    toc.lines()
    .filter_map(|l| toc_pattern.captures(l))
    .map(|link| {
        let level = if link.name("indent").unwrap().chars().count() == 0 { "#" } else { "##" };
        let id = filename_pattern.captures(
            link.name("filename").unwrap()
        ).unwrap().name("name").unwrap();
        
        let headline = format!(
            "{level} {name} {{#sec--{link}}}\n",
            level = level, name = link.name("title").unwrap(), link = id
        );

        Chapter {
            file: link.name("filename").unwrap().to_string(),
            headline: headline,
        }
    })
    .collect::<Vec<Chapter>>()
}
