#!/bin/sh
cargo build
cargo test

# TODO non debug, release

# Group binary with lipo
cargo +ios-arm64-nightly-2021-02-25 build --target aarch64-apple-ios --release --lib
cargo build --target x86_64-apple-ios
# Print NonFat -> Ok
lipo -info ./target/aarch64-apple-ios/debug/liblibastro.a
lipo -info ./target/x86_64-apple-ios/debug/liblibastro.a

# Group in one
lipo -create ./target/aarch64-apple-ios/debug/liblibastro.a ./target/x86_64-apple-ios/debug/liblibastro.a -output ./target/libastro.a
# Print architecture
lipo -info ./target/libastro.a
