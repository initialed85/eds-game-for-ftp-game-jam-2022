#!/usr/bin/env bash

set -e - x

function cleanup {
  kill %1
}
trap cleanup EXIT

echo -e "starting wasm-server-runner..."
wasm-server-runner ./client.wasm >/dev/null &

echo -e "\nwaiting for wasm-server-runner..."
while ! curl -s http://127.0.0.1:1334/ >/dev/null; do
  sleep 1
done

echo -e "\ndownloading .js and .wasm files..."
mkdir -p ./api/
curl -s http://127.0.0.1:1334/api/wasm.js >./api/wasm.js
# curl -s http://127.0.0.1:1334/api/wasm.wasm | brotli -d >./api/wasm.wasm
curl -s http://127.0.0.1:1334/api/wasm.wasm >./api/wasm.wasm

echo -e "\ndone; extracted:\n"

ls -al ./api/wasm.*
