cargo install grcov
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
export RUSTDOCFLAGS="-Cpanic=abort"
cargo clean;
cargo update;
cargo build;
cargo test --features "integration" -- --force-run-in-process -Z unstable-options;
~/.cargo/bin/grcov ./target/debug/ -s . -t lcov --llvm --branch --ignore-not-existing --ignore "/*" --ignore "*/tests/*" --ignore "*/dto/*" --ignore "*/domain_value/*" -o lcov.info;
genhtml -o ./target/debug/coverage/ --branch-coverage --show-details --highlight --ignore-errors source --legend ./lcov.info
rm ./lcov.info
