#!/bin/bash
set -euxo pipefail
cd "$(dirname "$0")"
cd front
volta run yarn
volta run yarn build
cd ../
cargo run --release -- -p 3030
