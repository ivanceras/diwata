FROM ubuntu:16.04

## Install postgresql

RUN apt-get update

RUN apt-get install -y --force-yes postgresql-9.5 postgresql-client-9.5 postgresql-contrib-9.5

RUN apt-get install -y curl rsync

RUN curl -sL https://deb.nodesource.com/setup_8.x | bash -s
RUN apt-get install -y nodejs

RUN mkdir -p ~/.npm \
    npm config set prefix ~/.npm \
    export PATH="$PATH:$HOME/.npm/bin"\ 
    npm install -g elm@0.18 \
    npm install -g google-closure-compiler-js

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly

RUN git clone https://github.com/ivanceras/diwata \
    cd diwata \
    git submodule update --init  --recursive

CMD run_sakila.sh
