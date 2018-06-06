#!/bin/sh

find book_src -type f \( -iname "*.md" ! -iname "SUMMARY.md" \) -exec sed -i '0,/# /{s/#/% /}' {} \;
find book_src -type f -name "SUMMARY.md" -exec sed -i 's/- \[/* \[/g' {} \;
find book_src -type f -name "SUMMARY.md" -exec sed -i 's/^\[/* \[/g' {} \;
