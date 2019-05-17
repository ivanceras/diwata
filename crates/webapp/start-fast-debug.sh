#!/bin/bash

if wasm-pack build --target no-modules --release; then
    basic-http-server ./ -a 0.0.0.0:6001
fi

