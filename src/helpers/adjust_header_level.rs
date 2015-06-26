use std::error::Error;
use std::iter::repeat;
use regex::Regex;

const CODE_BLOCK_TOGGLE: &'static str = "```";

pub type HeaderLevel = i32;

pub fn adjust_header_level(input: &str, base_level: HeaderLevel) -> Result<String, Box<Error>> {
    let headline_pattern = Regex::new(r"(?x)
        ^
        (?P<level>[\x23]+)  # A bunch of hash symbols
        \s
        (?P<title>.+)       # Title, and maybe id
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

        if let Some(headline) = headline_pattern.captures(line) {
            // level := number of '#'s.
            // '#' is always 1 byte, so .len() is save to use.
            let level = headline.name("level").unwrap().len() as i32;
            let new_level = calc_header_level(base_level, level);
            let new_headline = format!(
                "{level} {title}\n",
                level = repeat("#").take(new_level as usize).collect::<String>(),
                title = headline.name("title").unwrap()
            );
            return initial + &new_headline;
        }

        initial + line + "\n"
    });

    Ok(output)
}

fn calc_header_level(base_level: HeaderLevel, current_level: HeaderLevel) -> HeaderLevel {
    current_level + base_level - 1
}

#[test]
fn header_level_calculation() {
    //                           base level | current level | new level
    assert_eq!(calc_header_level(1,           1),             1);
    assert_eq!(calc_header_level(1,           2),             2);
    assert_eq!(calc_header_level(2,           2),             3);
    assert_eq!(calc_header_level(2,           1),             2);
}
