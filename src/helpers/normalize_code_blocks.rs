use std::error::Error;
use regex::Regex;

use helpers::line_breaks;

const CODE_BLOCK_TOGGLE: &'static str = "```";

pub fn break_code_blocks(input: &str, max_len: usize, sep: &str) -> Result<String, Box<Error>> {
    let mut in_code_block = false;

    let output = input.lines()
    .fold(String::new(), |initial, line| {
        match (in_code_block, line.starts_with(CODE_BLOCK_TOGGLE)) {
            (true,  false) => {
                let lines = line_breaks::break_long_line(line, max_len, sep).unwrap();
                return initial + &lines + "\n";
            }
            (true,  true ) => { in_code_block = false; }
            (false, true ) => { in_code_block = true; }
            (false, false) => {}
        };
        initial + line + "\n"
    });

    Ok(output)
}


pub fn normalize_code_start(input: &str) -> Result<String, Box<Error>> {
    let rust_code_block_start = Regex::new(r"^```(.*)rust(.*)").unwrap();
    let hidden_code = Regex::new(r"^# ").unwrap();

    let mut in_code_block = false;

    let output = input.lines()
    .fold(String::new(), |initial, line| {
        if in_code_block && hidden_code.is_match(line) {
            initial
        } else if rust_code_block_start.is_match(line) {
            in_code_block = true;
            initial + "```rust\n"
        } else if line.starts_with(CODE_BLOCK_TOGGLE) {
            in_code_block = false;
            initial + line + "\n"
        } else {
            initial + line + "\n"
        }
    });

    Ok(output)
}

#[test]
fn code_block_breaking() {
    let long_code_block = "If we truly want a reference, we need the other option: ensure that our reference goes out of scope before we try to do the mutation. That looks like this:

```text
Whew! The Rust compiler gives quite detailed errors at times, and this is one of those times. As the error explains, while we made our binding mutable, we still cannot call `push`. This is because we already have a reference to an element of the vector, `y`. Mutating something while another reference exists is dangerous, because we may invalidate the reference. In this speciffic case, when we create the vector, we may have only allocated space for three elements. Adding a fourth would mean allocating a new chunk of memory for all thosee elements, copying the old values over, and updating the internal pointer to that memory. That all works just fine.
```

We created an inner scope with an additional set of curly braces. `y` will go out of scope before we call `push()`, and so we’re all good.";

    let code_block_broken_down = "If we truly want a reference, we need the other option: ensure that our reference goes out of scope before we try to do the mutation. That looks like this:

```text
Whew! The Rust compiler gives quite detailed errors at times, and this is one o
↳ f those times. As the error explains, while we made our binding mutable, we s
↳ till cannot call `push`. This is because we already have a reference to an el
↳ ement of the vector, `y`. Mutating something while another reference exists i
↳ s dangerous, because we may invalidate the reference. In this speciffic case,
↳  when we create the vector, we may have only allocated space for three elemen
↳ ts. Adding a fourth would mean allocating a new chunk of memory for all those
↳ e elements, copying the old values over, and updating the internal pointer to
↳  that memory. That all works just fine.
```

We created an inner scope with an additional set of curly braces. `y` will go out of scope before we call `push()`, and so we’re all good.
";

    let max_len = 80;

    let broken = break_code_blocks(long_code_block, max_len, "↳ ").unwrap();

    assert_eq!(broken, code_block_broken_down);
}

#[test]
fn code_block_starts() {
    let code_blocks = "Code:

```sh
$ lol
```

```{rust,ignore}
let x = true;
```

``` rust,no_extras
let x = true;
```

```rust
# use magic::from_the_future::*;
let x = true;
```
";

    let code_blocks_clean = "Code:

```sh
$ lol
```

```rust
let x = true;
```

```rust
let x = true;
```

```rust
let x = true;
```
";

    let cleaned = normalize_code_start(code_blocks).unwrap();

    assert_eq!(cleaned, code_blocks_clean);
}
