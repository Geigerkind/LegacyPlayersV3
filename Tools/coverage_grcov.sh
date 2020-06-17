cargo install grcov
cargo clean;
cargo update;

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests"
export RUSTDOCFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests"
cargo test --workspace --all-features --no-fail-fast -- --test-threads=32;
~/.cargo/bin/grcov ./target/debug/ -s . -t lcov --llvm --branch --ignore-not-existing --filter "covered" --ignore "/*" --ignore "*/tests/*" --ignore "*/dto/*" --ignore "*/domain_value/*" --ignore "*/main.rs" --excl-br-line "#\\[\\w+(\\([\\w\",/\\s=<>]+\\))?\\]|pub\\s\\w+:[\\w<,\\s>]+," --excl-line "#\\[\\w+(\\([\\w\",/\\s=<>]+\\))?\\]" -o lcov.info;
genhtml -o ./target/debug/coverage/ --branch-coverage --show-details --highlight --ignore-errors source --legend ./lcov.info
rm ./lcov.info
