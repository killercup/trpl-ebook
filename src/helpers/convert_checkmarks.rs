use regex::Regex;

pub fn convert_checkmarks(input: &str) -> String {
    let checks = Regex::new("[\u{2713}\u{2714}]").unwrap();
    // rely on pandoc's raw latex support
    checks.replace_all(input, "\\checkmark")
}

#[test]
fn checkmark_conversion() {
    assert_eq!(convert_checkmarks("checks: ✓ ✔"), "checks: \\checkmark \\checkmark");
}
