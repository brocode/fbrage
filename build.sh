#!/bin/bash

set -e -o pipefail -u

pushd rage-webassembly
wasm-pack build --release
popd

pnpm build

podman build . -t docker.io/bomgar/fbrage
