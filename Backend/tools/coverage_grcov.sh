cargo install grcov
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
cargo clean;
cargo update;
cargo build;
cargo test --features "integration";
~/.cargo/bin/grcov ./target/debug/ -s . -t lcov --llvm --branch --ignore-not-existing --ignore "/*" --ignore "src/tests/*" -o lcov.info;
genhtml -o ./target/debug/coverage/ --branch-coverage --show-details --highlight --ignore-errors source --legend ./lcov.info
rm ./lcov.info