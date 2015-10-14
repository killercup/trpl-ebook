#! /bin/sh

RUN="cargo run --release --"

$RUN --prefix=trpl --source=trpl --meta=trpl_meta.yml && \
$RUN --prefix=nomicon --source=nomicon --meta=nomicon_meta.yml
$RUN --prefix=tlborm --source=tlborm --meta=tlborm_meta.yml
