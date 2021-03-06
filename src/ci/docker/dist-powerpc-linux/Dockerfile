FROM ubuntu:16.04

RUN apt-get update && apt-get install -y --no-install-recommends \
  automake \
  bison \
  bzip2 \
  ca-certificates \
  cmake \
  curl \
  file \
  flex \
  g++ \
  gawk \
  gdb \
  git \
  gperf \
  help2man \
  libncurses-dev \
  libtool-bin \
  make \
  patch \
  python2.7 \
  sudo \
  texinfo \
  wget \
  xz-utils \
  libssl-dev \
  pkg-config

RUN curl -OL https://github.com/Yelp/dumb-init/releases/download/v1.2.0/dumb-init_1.2.0_amd64.deb && \
    dpkg -i dumb-init_*.deb && \
    rm dumb-init_*.deb
ENTRYPOINT ["/usr/bin/dumb-init", "--"]

# Ubuntu 16.04 (this contianer) ships with make 4, but something in the
# toolchains we build below chokes on that, so go back to make 3
RUN curl https://ftp.gnu.org/gnu/make/make-3.81.tar.gz | tar xzf - && \
      cd make-3.81 && \
      ./configure --prefix=/usr && \
      make && \
      make install && \
      cd .. && \
      rm -rf make-3.81

RUN curl http://crosstool-ng.org/download/crosstool-ng/crosstool-ng-1.22.0.tar.bz2 | \
      tar xjf - && \
      cd crosstool-ng && \
      ./configure --prefix=/usr/local && \
      make -j$(nproc) && \
      make install && \
      cd .. && \
      rm -rf crosstool-ng

RUN groupadd -r rustbuild && useradd -m -r -g rustbuild rustbuild
RUN mkdir /x-tools && chown rustbuild:rustbuild /x-tools
USER rustbuild
WORKDIR /tmp

COPY dist-powerpc-linux/patches/ /tmp/patches/
COPY dist-powerpc-linux/powerpc-linux-gnu.config dist-powerpc-linux/build-powerpc-toolchain.sh /tmp/
RUN ./build-powerpc-toolchain.sh

USER root

RUN curl -o /usr/local/bin/sccache \
      https://s3.amazonaws.com/rust-lang-ci/rust-ci-mirror/2017-05-12-sccache-x86_64-unknown-linux-musl && \
      chmod +x /usr/local/bin/sccache

ENV PATH=$PATH:/x-tools/powerpc-unknown-linux-gnu/bin

ENV \
    CC_powerpc_unknown_linux_gnu=powerpc-unknown-linux-gnu-gcc \
    AR_powerpc_unknown_linux_gnu=powerpc-unknown-linux-gnu-ar \
    CXX_powerpc_unknown_linux_gnu=powerpc-unknown-linux-gnu-g++

ENV HOSTS=powerpc-unknown-linux-gnu

ENV RUST_CONFIGURE_ARGS --host=$HOSTS --enable-extended
ENV SCRIPT python2.7 ../x.py dist --host $HOSTS --target $HOSTS

# FIXME(#36150) this will fail the bootstrap. Probably means something bad is
#               happening!
ENV NO_LLVM_ASSERTIONS 1
