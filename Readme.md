# 'The Rust Programming Language' as EBook

This repository contains stuff to convert [this book](http://doc.rust-lang.org/book/) to HTML, EPUB and PDF.

**[Download Links](http://killercup.github.io/trpl-ebook/)**

[![Build Status](https://travis-ci.org/killercup/trpl-ebook.svg?branch=master)](https://travis-ci.org/killercup/trpl-ebook)

## DIY

Install:

- pandoc
- Rust and cargo
- XeLaTeX, up to date (`sudo tlmgr update -all`) and probably some additional packages (`sudo tlmgr install $pkg`) such as:
    + framed
    + hyphenat
    + quotchap
    + collection-fontsrecommended
- the DejaVu Sans Mono font: http://dejavu-fonts.org/
- the IPA font for Japanese Text: http://ipafont.ipa.go.jp/ipaexfont/download.html#en

Then run:

```sh
$ cargo run --release
```

Voilà!

## Build different books

There are some CLI arguments that you can use to compile books other than the default (`trpl`). E.g., this repository also include the Rustonomicon.

You can build it like this:

```sh
$ cargo run --release -- --prefix=nomicon --source=nomicon --meta=nomicon_meta.yml
```

## License

The book content itself as well as any code I added as part of this repository is Copyright (c) 2015 The Rust Project Developers and licensed like Rust itself ([MIT](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT) and [Apache](https://github.com/rust-lang/rust/blob/master/LICENSE-APACHE)).
