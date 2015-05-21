use std::error::Error;
use std::fmt;

use std::io::prelude::*;
use std::process::{self, Command, Stdio};

#[derive(Debug, Hash, PartialEq, Eq)]
enum CommandError {
    StdIn,
    StdOut
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CommandError::StdIn  => write!(f, "Error getting stdin"),
            CommandError::StdOut => write!(f, "Error getting stdout"),
        }
    }
}

impl Error for CommandError {
    fn description(&self) -> &str {
        match *self {
            CommandError::StdIn  => "Error getting stdin",
            CommandError::StdOut => "Error getting stdout"
        }
    }
}

pub fn run(command: &str, args: &str, input: &str) -> Result<String, Box<Error>> {
    let args: Vec<&str> = if args.is_empty() {
        vec![]
    } else {
        // Command arguments are space separated but may contain sub strings in quotation marks
        let mut in_substr = false;
        args.split(|c: char| {
            if c == '\'' { in_substr = !in_substr; }
            !in_substr && (c == ' ')
        }).collect()
    };

    let process = try!(
        Command::new(command)
                .args(&args)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
    );

    {
        let mut stdin: process::ChildStdin = try!(process.stdin.ok_or(CommandError::StdIn));
        try!(stdin.write_all(input.as_bytes()));
    }

    let mut output = String::new();

    let mut stdout: process::ChildStdout = try!(process.stdout.ok_or(CommandError::StdOut));
    try!(stdout.read_to_string(&mut output));

    Ok(output)
}

#[test]
fn dry_run() {
    let output = run("cat", "", "lol").unwrap();

    assert_eq!(output, "lol");
}
