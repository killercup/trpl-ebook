FROM ubuntu:16.04

RUN \
  apt-get update && \
  apt-get -y upgrade && \
  apt-get install -y build-essential && \
  apt-get install -y software-properties-common && \
  apt-get install -y curl git htop unzip vim wget fontconfig && \
  apt-get install -y texlive texlive-latex-extra texlive-generic-extra  texlive-xetex && \
  apt-get install -y pandoc && \
  apt-get install -y ttf-dejavu && \
  curl -o setup.sh  https://sh.rustup.rs -sS && \
  sh setup.sh -y && \
  rm -rf /var/lib/apt/lists/*

ENV PATH="~/.cargo/bin:${PATH}"

RUN \
  curl -o ipafont.zip https://oscdl.ipa.go.jp/IPAexfont/IPAexfont00301.zip && \
  unzip -j ipafont.zip -d ~/.fonts && \
  fc-cache -fv

ADD . /trpl-ebook
WORKDIR /trpl-ebook
CMD ["bash"]
