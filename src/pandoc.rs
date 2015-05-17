use std::error::Error;
use shell_pipe;

pub fn run(args: &str, input: &str) -> Result<String, Box<Error>> {
    shell_pipe::run("pandoc", args, input)
}

#[test]
fn dry_run() {
    let output = run("--from=markdown --base-header-level=2 --to=markdown", "# Hi there!").unwrap();

    assert_eq!(
        output,
"Hi there!
---------
"
    );
}
