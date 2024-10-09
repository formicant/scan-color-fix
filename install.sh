#! /bin/bash

cargo build --release
cp target/release/scan-color-fix ~/.local/bin/
cp script/scan.sh ~/.local/bin/scan
