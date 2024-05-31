#! /bin/bash

TARGETS=(aarch64-apple-darwin x86_64-apple-darwin)

for target in ${TARGETS[@]}; do
  filename=sourcemap-explorer-$target.tar.gz
  cargo build --release --target $target
  tar -cvf $filename -C ./target/$target/release smx
  shasum --algorithm 256 $filename
done
