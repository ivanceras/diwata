#!/bin/bash

wasm-pack build --target no-modules --release  -- --features "console_error_panic_hook"
basic-http-server ./ -a 0.0.0.0:6001

