#!/bin/bash
REPO_URL=https://github.com/rust-lang/rust/archive

die() {
    echo "$@" 1>&2
    exit 1
}

BRANCH=$1
if [ -z $BRANCH ]; then
    BRANCH=master
fi
URL=$REPO_URL/$BRANCH.tar.gz
rm -rf tmp
mkdir tmp
curl -L $URL | tar xz -C tmp || die "Failed to download rust source code"
rm -rf trpl nomicon
mv ./tmp/rust-*/src/doc/book trpl
mv ./tmp/rust-*/src/doc/nomicon nomicon
rm -rf tmp
