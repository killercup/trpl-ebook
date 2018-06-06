# 'The Rust Programming Language' as EBook

This repository contains stuff to convert [this book](http://doc.rust-lang.org/book/) to HTML, EPUB and PDF.

**[Download Links](http://killercup.github.io/trpl-ebook/)**

[![Build Status](https://travis-ci.org/killercup/trpl-ebook.svg?branch=master)](https://travis-ci.org/killercup/trpl-ebook)

## Docker
The simplest way to execute this is to use docker. With a installed and running docker service you can use the Makefile to build all books: 

```sh
$ make all
```
Or just build a specific book (rustonomicon, trpl or trpl2):

```sh
$ make trpl
```

Results will appear into the *dist* directory.


## DIY

Install:

- pandoc version >= 2.0
- Rust and cargo
- XeLaTeX, up to date (`sudo tlmgr update -all`) and probably some additional packages (`sudo tlmgr install $pkg`) such as:
    + framed
    + hyphenat
    + quotchap
    + collection-fontsrecommended
    + mathspec
    + euenc
    + xltxtra
    + xecjk
    + fancyhdr
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
$ cargo run --release -- --prefix=rustonomicon --source=book_src/nomicon
```

```sh
$ cargo run --release -- --source=book_src/trpl
```
If your books meta.yml is not in the document directory supply the `--meta=$path-to-file` argument.

## License

The book content itself as well as any code I added as part of this repository is Copyright (c) 2015 The Rust Project Developers and licensed like Rust itself ([MIT](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT) and [Apache](https://github.com/rust-lang/rust/blob/master/LICENSE-APACHE)).
