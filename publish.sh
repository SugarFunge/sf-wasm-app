#!/bin/bash

APPNAME=sf-wasm-app

set -e

cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/${APPNAME}.wasm

rsync -a assets/ out/assets/
rsync -a index.html out/index.html

wasm-opt -Os -o out/${APPNAME}_bg.wasm out/${APPNAME}_bg.wasm

rsync -r out/ ${APPNAME}:/root/${APPNAME}/
