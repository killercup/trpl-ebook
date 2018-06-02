#! /bin/sh

RUN="cargo run --release --"

$RUN --source=book_src/trpl && \
$RUN --source=book_src/trpl2 && \
$RUN --source=book_src/nomicon
