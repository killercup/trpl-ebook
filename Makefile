help:
	@echo "Compile all books to ./dist use 'make all'"
	@echo "Compile a specific book to ./dist use 'make bookname'. Booknames are rustonomicon, trpl and trpl2"

MAKEFILE_DIR?=$(shell pwd)
LOCAL_DIST_DIR=$(MAKEFILE_DIR)/dist
DIST_DIR=/trpl-ebook/dist

build:
	docker build -t trpl-ebook -f Dockerfile .

interactive: build
	docker run --rm -it -v $(MAKEFILE_DIR):/trpl-ebook trpl-ebook bash

all: build
	docker run --rm -it -v $(LOCAL_DIST_DIR):$(DIST_DIR) trpl-ebook bash build.sh

rustonomicon: build
	docker run --rm -it -v $(LOCAL_DIST_DIR):$(DIST_DIR) trpl-ebook bash -c "cargo run -- --prefix=rustonomicon --source=book_src/nomicon"

trpl: build
	docker run --rm -it -v $(LOCAL_DIST_DIR):$(DIST_DIR) trpl-ebook bash -c "cargo run -- --source=book_src/trpl"

trpl2: build
	docker run --rm -it -v $(LOCAL_DIST_DIR):$(DIST_DIR) trpl-ebook bash -c "cargo run -- --source=book_src/trpl2"
