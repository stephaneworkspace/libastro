#!/bin/sh
cargo build
cargo test

# Group binary with lipo
cargo build --target aarch64-apple-ios
cargo build --target x86_64-apple-ios
# Print NonFat -> Ok
lipo -info ./target/aarch64-apple-ios/debug/liblibastro.a
lipo -info ./target/x86_64-apple-ios/debug/libibastro.a

# Group in one
lipo -create ./target/aarch64-apple-ios/debug/liblibastro.a ./target/x86_64-apple-ios/debug/liblibastro.a -output ./target/libastro.a
# Print architecture
lipo -info ./target/libastro.a
