#!/bin/bash

wasm-pack build --target no-modules

basic-http-server ./ -a 0.0.0.0:6001

