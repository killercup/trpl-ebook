#! env ruby

# The date of the src/ files
RELEASE_DATE = "2015-05-15"

TOC_LINK_REGEX = /(?<indent>\s*?)\* \[(?<title>.+?)\]\((?<filename>.+?)\)/

HIDDEN_CODE = Regexp.new("^# ")
RUST_CODE_START = Regexp.new("^```(.*)rust(.*)")
CODE_BLOCK_START = CODE_BLOCK_END = Regexp.new("^```")

MARKDOWN_OPTIONS = "markdown+grid_tables+pipe_tables+raw_html+implicit_figures+footnotes+intraword_underscores+auto_identifiers-inline_code_attributes"

def break_long_lines(line, max_len=87, sep="↳ ")
    return line if line.length <= max_len

    output = ""

    cursor = max_len
    output << line[0..cursor - 1]
    while line.length > cursor
        new_cursor = cursor + (max_len - sep.length)
        output << "\n"
        output << sep
        output << line[cursor..new_cursor - 1]
        cursor = new_cursor
    end
    output
end

def break_long_code_lines(input)
    in_code_block = false

    input
    .lines.reduce "" do |initial, line|
        if in_code_block && line.match(CODE_BLOCK_END)
            in_code_block = false
            initial + line
        elsif in_code_block
            initial + break_long_lines(line)
        elsif line.match(CODE_BLOCK_START)
            in_code_block = true
            initial + line
        else
            initial + line
        end
    end
end

# cf. http://stackoverflow.com/a/29115826/1254484
EMOJIS = /[\u{203C}\u{2049}\u{20E3}\u{2122}\u{2139}\u{2194}-\u{2199}\u{21A9}-\u{21AA}\u{231A}-\u{231B}\u{23E9}-\u{23EC}\u{23F0}\u{23F3}\u{24C2}\u{25AA}-\u{25AB}\u{25B6}\u{25C0}\u{25FB}-\u{25FE}\u{2600}-\u{2601}\u{260E}\u{2611}\u{2614}-\u{2615}\u{261D}\u{263A}\u{2648}-\u{2653}\u{2660}\u{2663}\u{2665}-\u{2666}\u{2668}\u{267B}\u{267F}\u{2693}\u{26A0}-\u{26A1}\u{26AA}-\u{26AB}\u{26BD}-\u{26BE}\u{26C4}-\u{26C5}\u{26CE}\u{26D4}\u{26EA}\u{26F2}-\u{26F3}\u{26F5}\u{26FA}\u{26FD}\u{2702}\u{2705}\u{2708}-\u{270C}\u{270F}\u{2712}\u{2714}\u{2716}\u{2728}\u{2733}-\u{2734}\u{2744}\u{2747}\u{274C}\u{274E}\u{2753}-\u{2755}\u{2757}\u{2764}\u{2795}-\u{2797}\u{27A1}\u{27B0}\u{2934}-\u{2935}\u{2B05}-\u{2B07}\u{2B1B}-\u{2B1C}\u{2B50}\u{2B55}\u{3030}\u{303D}\u{3297}\u{3299}\u{1F004}\u{1F0CF}\u{1F170}-\u{1F171}\u{1F17E}-\u{1F17F}\u{1F18E}\u{1F191}-\u{1F19A}\u{1F1E7}-\u{1F1EC}\u{1F1EE}-\u{1F1F0}\u{1F1F3}\u{1F1F5}\u{1F1F7}-\u{1F1FA}\u{1F201}-\u{1F202}\u{1F21A}\u{1F22F}\u{1F232}-\u{1F23A}\u{1F250}-\u{1F251}\u{1F300}-\u{1F320}\u{1F330}-\u{1F335}\u{1F337}-\u{1F37C}\u{1F380}-\u{1F393}\u{1F3A0}-\u{1F3C4}\u{1F3C6}-\u{1F3CA}\u{1F3E0}-\u{1F3F0}\u{1F400}-\u{1F43E}\u{1F440}\u{1F442}-\u{1F4F7}\u{1F4F9}-\u{1F4FC}\u{1F500}-\u{1F507}\u{1F509}-\u{1F53D}\u{1F550}-\u{1F567}\u{1F5FB}-\u{1F640}\u{1F645}-\u{1F64F}\u{1F680}-\u{1F68A}]/


def removeEmoji(input)
    input.gsub(EMOJIS, '')
end

def normalizeCodeSnipetts(input)
    in_code_block = false

    input
    .lines.reduce "" do |initial, line|
        if in_code_block and line.match(HIDDEN_CODE)
            # skip line
            initial
        elsif line.match(RUST_CODE_START)
            in_code_block = true
            # normalize code block start
            initial + "```rust\n"
        elsif line.match(CODE_BLOCK_END)
            in_code_block = false
            initial + "```\n"
        else
            initial + line
        end
    end
end

def normalize_title(title)
    # Some chapter titles start with Roman numerals, e.g. "I: The Basics"
    title.sub /(([IV]+):\s)/, ''
end

def normalizeLinks(input)
    input
    .gsub("../std", "http://doc.rust-lang.org/std")
    .gsub("../reference", "http://doc.rust-lang.org/reference")
    .gsub("../rustc", "http://doc.rust-lang.org/rustc")
    .gsub("../syntax", "http://doc.rust-lang.org/syntax")
    .gsub("../core", "http://doc.rust-lang.org/core")
    .gsub(/\]\(([\w\-\_]+)\.html\)/, '](#sec--\1)') # internal links: each file begins with <hX id="#sec-FILEANME">TITLE</hX>
end

def pandoc(file, header_level=3)
    normalizeTables = 'sed -E \'s/^\+-([+-]+)-\+$/| \1 |/\''

    normalizeCodeSnipetts normalizeLinks `cat #{file} | #{normalizeTables} | pandoc --from=#{MARKDOWN_OPTIONS} --to=#{MARKDOWN_OPTIONS} --base-header-level=#{header_level} --indented-code-classes=rust --atx-headers`
end

book = <<-eos
---
title: "The Rust Programming Language"
author: "The Rust Team"
date: #{RELEASE_DATE}
description: "This book will teach you about the Rust Programming Language. Rust is a modern systems programming language focusing on safety and speed. It accomplishes these goals by being memory safe without using garbage collection."
language: en
documentclass: book
links-as-notes: true
verbatim-in-note: true
toc-depth: 2
...

eos

book << "# Introduction\n\n"
book << pandoc("src/README.md", 1)
book << "\n\n"

File.open("src/SUMMARY.md", "r").each_line do |line|
    link = TOC_LINK_REGEX.match(line)
    if link
        level = link[:indent].length == 0 ? "#" : "##"
        book << "#{level} #{normalize_title link[:title]} {#sec--#{File.basename(link[:filename], '.*')}}\n\n"
        book << pandoc("src/#{link[:filename]}")
        book << "\n\n"
    end
end

File.open("dist/trpl-#{RELEASE_DATE}.md", "w") { |file|
    file.write(book)
}

`pandoc dist/trpl-#{RELEASE_DATE}.md --from=#{MARKDOWN_OPTIONS} --smart --normalize --standalone --self-contained --highlight-style=tango --table-of-contents --template=lib/template.html --css=lib/pandoc.css --to=html5 --output=dist/trpl-#{RELEASE_DATE}.html`
puts "[✓] HTML"

`pandoc dist/trpl-#{RELEASE_DATE}.md --from=#{MARKDOWN_OPTIONS} --smart --normalize --standalone --self-contained --highlight-style=tango --epub-stylesheet=lib/epub.css --table-of-contents --output=dist/trpl-#{RELEASE_DATE}.epub`
puts "[✓] EPUB"

# again, with shorter code lines
File.open("dist/trpl-#{RELEASE_DATE}.md", "w") { |file|
    file.write(removeEmoji break_long_code_lines book)
}

`pandoc dist/trpl-#{RELEASE_DATE}.md --from=#{MARKDOWN_OPTIONS} --smart --normalize --standalone --self-contained --highlight-style=tango --chapters --table-of-contents --variable papersize='a4paper' --variable monofont='DejaVu Sans Mono' --template=lib/template.tex --latex-engine=xelatex --to=latex --output=dist/trpl-#{RELEASE_DATE}-a4.pdf`
puts "[✓] PDF (A4)"

`pandoc dist/trpl-#{RELEASE_DATE}.md --from=#{MARKDOWN_OPTIONS} --smart --normalize --standalone --self-contained --highlight-style=tango --chapters --table-of-contents --variable monofont='DejaVu Sans Mono' --variable papersize='letterpaper' --template=lib/template.tex --latex-engine=xelatex --to=latex --output=dist/trpl-#{RELEASE_DATE}-letter.pdf`
puts "[✓] PDF (Letter)"

# back to original line length
File.open("dist/trpl-#{RELEASE_DATE}.md", "w") { |file|
    file.write(book)
    puts "[✓] Markdown"
}

FILE_PREFIX = /^trpl-(?<date>(\d+)-(\d+)-(\d+))/
FILE_NAME = /^trpl-(?<date>(\d+)-(\d+)-(\d+))(?<name>.*)/

file_listing = Dir["dist/trpl*"]
    .map{|f| f.gsub("dist/", "") }
    .sort.reverse
    .group_by {|f| f[FILE_PREFIX] }
    .reject
    .map {|prefix, files|
        html = "<li><h2>#{prefix.match(FILE_PREFIX)[:date]}</h2><ul>"
        html << files.map {|file|
            "<li><a href='#{file}'>#{
                file.match(FILE_NAME)[:name].gsub('-', '').gsub('.', ' ').upcase.strip
            }</a></li>"
        }.join("\n")
        html << "</ul></li>"
        html
    }.join("\n")

index = <<-eos
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Download 'The Rust Programming Language' E-Books (PDF, EPUB, MOBI)</title>
    <meta name="viewport" content="width=device-width"/>
    <style>
        body { max-width: 40em; margin: 6em auto 2em; font-size: 16px; font-family: sans-serif; line-height: 1.3; padding: 0.6em; }
        article, header, footer, aside { display: block; }
        li { margin-bottom: 0.5em; }
        footer { text-align: center; margin-top: 4em; font-size: 0.8em; }
        aside { position: absolute; top: 0; right: 0; border: 0; }
        aside img { border: 0; }
    </style>
</head>
<body>
    <article role="main">
        <header>
            <h1>'The Rust Programming Language' E-Books</h1>
        </header>
        <ul>
            <li>
                <strong><a href="http://doc.rust-lang.org/book/">The original on rust-lang.org</a></strong>
            </li>
            #{file_listing}
        </ul>
        <footer>
            Made with ♥︎, too much RegEx and <a href="http://pandoc.org/">Pandoc</a> by <a href="http://pascalhertleif.de/">Pascal Hertleif</a>
        </footer>
    </article>
    <aside>
        <a href="https://github.com/killercup/trpl-ebook">
            <img src="https://s3.amazonaws.com/github/ribbons/forkme_right_gray_6d6d6d.png" alt="Fork me on GitHub"/>
        </a>
    </aside>
</body>
</html>
eos

File.open("dist/index.html", "w") { |file|
    file.write(index)
    puts "[✓] Index page"
}
