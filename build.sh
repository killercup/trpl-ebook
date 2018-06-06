#! /bin/sh

RUN="cargo run --"

sh ./adjust_book_src.sh

$RUN --source=book_src/trpl && \
$RUN --source=book_src/trpl2 && \
$RUN --source=book_src/nomicon
