pub const RELEASE_DATE: &'static str = "2017-06-12";

pub const MARKDOWN: &'static str = "markdown+grid_tables+pipe_tables-simple_tables+raw_html+implicit_figures+footnotes+intraword_underscores+auto_identifiers-inline_code_attributes";

pub const HTML: &'static str = "--smart --normalize --standalone --self-contained --highlight-style=tango --table-of-contents --section-divs --template=lib/template.html --css=lib/pandoc.css --to=html5";

pub const EPUB: &'static str = "--smart --normalize --standalone --self-contained --highlight-style=tango --epub-stylesheet=lib/epub.css --table-of-contents";

pub const LATEX: &'static str = "--smart --normalize --standalone --self-contained --highlight-style=tango --top-level-division=chapter --table-of-contents --template=lib/template.tex --latex-engine=xelatex --to=latex";
