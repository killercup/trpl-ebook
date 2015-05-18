use std::error::Error;
use std::ascii::AsciiExt;

use helpers::shell_pipe;
use convert_book::options;

pub fn run(args: &str, input: &str) -> Result<String, Box<Error>> {
    shell_pipe::run("pandoc", args, input)
}

pub fn save_as(book: &str, format: &str, opts: &str) -> Result<(), Box<Error>> {
    let opts = format!(
        "--from={markdown_opts} {opts} --output=dist/trpl-{release_date}.{format}",
        markdown_opts = options::MARKDOWN,
        opts = opts,
        release_date = options::RELEASE_DATE,
        format = format
    );

    try!(run(&opts, &book));

    println!("[✓] {}", format.to_ascii_uppercase());

    Ok(())
}

#[test]
fn dry_run() {
    let output = run(
        "--from=markdown --base-header-level=2 --to=markdown --atx-headers",
        "# Hi there!\n"
    ).unwrap();

    assert_eq!(output, "## Hi there!\n");
}
