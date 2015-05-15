# 'The Rust Programming Language' as EBook

This repository contains stuff to convert [this book](http://doc.rust-lang.org/book/) to HTML, EPUB and PDF.

**[Download Links](http://killercup.github.io/trpl-ebook/)**

## DIY

Install:

- pandoc
- ruby
- XeLaTeX and probably some additional packages, I had to install (`sudo tlmgr install $pkg`) those:
    + framed
    + hyphenat
    + quotchap
- the DejaVu Sans Mono font

Then run:

```sh
$ ruby build.rb
```

Voilá!

## License

The book content itself as well as any code I added as part of this repository is Copyright (c) 2015 The Rust Project Developers and licensed like Rust itself ([MIT](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT) and [Apache](https://github.com/rust-lang/rust/blob/master/LICENSE-APACHE)).
