FROM ubuntu:16.04

RUN \
  apt-get update && \
  apt-get -y upgrade && \
  apt-get install -y build-essential && \
  apt-get install -y software-properties-common && \
  apt-get install -y curl git htop unzip vim wget fontconfig && \
  apt-get install -y texlive texlive-latex-extra texlive-generic-extra texlive-xetex && \
  apt-get install -y librsvg2-bin && \
  apt-get install -y ttf-dejavu && \
  curl -o setup.sh https://sh.rustup.rs -sS && \
  sh setup.sh -y && \
  rm -rf /var/lib/apt/lists/*

ENV PATH="~/.cargo/bin:${PATH}"

RUN \
  curl -o pandoc2.deb -L https://github.com/jgm/pandoc/releases/download/2.2.1/pandoc-2.2.1-1-amd64.deb && \
  file pandoc2.deb && \
  dpkg -i pandoc2.deb && \
  rm pandoc2.deb

RUN \
  curl -o ipafont.zip https://oscdl.ipa.go.jp/IPAexfont/IPAexfont00301.zip && \
  unzip -j ipafont.zip -d ~/.fonts && \
  rm ipafont.zip && \
  fc-cache -fv

# Workaround to build dependencies beforehand.

WORKDIR /
RUN USER=root ~/.cargo/bin/cargo new trpl-ebook
WORKDIR /trpl-ebook
COPY Cargo.toml /trpl-ebook
COPY Cargo.lock /trpl-ebook
RUN ~/.cargo/bin/cargo build
RUN ~/.cargo/bin/cargo clean -p compile-trpl


COPY . /trpl-ebook
RUN  chmod +x build.sh && chmod +x adjust_book_src.sh && ./adjust_book_src.sh
RUN ~/.cargo/bin/cargo build
CMD ["bash"]
