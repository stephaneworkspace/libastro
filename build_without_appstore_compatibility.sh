#!/bin/sh
cargo build
cargo test

# Group binary with lipo
cargo build --target aarch64-apple-ios
cargo build --target x86_64-apple-ios
# Print NonFat -> Ok
lipo -info ./target/aarch64-apple-ios/debug/liblibastro.so
lipo -info ./target/x86_64-apple-ios/debug/liblibastro.so

# Group in one
lipo -create ./target/aarch64-apple-ios/debug/liblibastro.so ./target/x86_64-apple-ios/debug/liblibastro.so -output ./target/libastro.so
# Print architecture
lipo -info ./target/libastro.so
