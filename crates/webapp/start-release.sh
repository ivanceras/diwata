#!/bin/bash

wasm-pack build --target no-modules --release 
basic-http-server ./ -a 0.0.0.0:6001

