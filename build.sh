#! /bin/sh

RUN="cargo run --"

# $RUN --prefix=trpl-1st-ed --source=trpl-1st-edition --meta=trpl_meta.yml && \
$RUN --prefix=trpl-2nd-ed --source=trpl-2nd-edition --meta=trpl_meta.yml
# $RUN --prefix=nomicon --source=nomicon --meta=nomicon_meta.yml
