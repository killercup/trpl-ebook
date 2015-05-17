use std::error::Error;

pub fn break_long_line(line: &str, max_len: usize, sep: &str) -> Result<String, Box<Error>> {
    let sep_length = sep.chars().count() as usize;
    let mut output = String::with_capacity(line.len());

    // First time: `max_len`, after that `max_len - sep_length`
    let mut line_end = max_len;

    for (index, ch) in line.chars().enumerate() {
        if index >= (line_end - 1) {
            line_end += max_len - sep_length - 1;
            output.push_str("\n");
            output.push_str(sep);
        }
        output.push(ch);
    }

    Ok(output)
}

#[test]
fn break_long_lines() {
    let long_line = "markdown+grid_tables+pipe_tables+raw_html+implicit_figures+footnotes+intraword_underscores+auto_identifiers-inline_code_attributesmarkdown+grid_tables+pipe_tables+raw_html+implicit_figures+footnotes+intraword_underscores+auto_identifiers-inline_code_attributes";

    let correct_split = "markdown+grid_tables+pipe_tables+raw_html+implicit_figures+footnotes+intraword_
↳ underscores+auto_identifiers-inline_code_attributesmarkdown+grid_tables+pipe_
↳ tables+raw_html+implicit_figures+footnotes+intraword_underscores+auto_identif
↳ iers-inline_code_attributes";

    let max_len = 80;

    let broken = break_long_line(long_line, max_len, "↳ ").unwrap();

    assert_eq!(broken, correct_split);
    assert!(broken.lines().all(|x| { x.chars().count() <= max_len }));
    assert_eq!(broken.lines().count(), 4);
}
