#!/bin/sh
sudo apt install -y nasm qemu-system-x86 xorriso
rustup component add rust-src
./scripts/add_rustfmt_hook.sh