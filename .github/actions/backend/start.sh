echo ${GITHUB_TOKEN}
git clone --single-branch --branch ${BRANCH_NAME} https://Geigerkind:${GITHUB_TOKEN}@github.com/Geigerkind/LegacyPlayersV3
cd LegacyPlayersV3/Backend
cp .env_ci .env
cargo install grcov

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
cargo build;
cargo test --features "integration";
~/.cargo/bin/grcov ./target/debug/ -s . -t lcov --llvm --branch --ignore-not-existing --ignore "/*" --ignore "src/tests/*" -o lcov.info;
bash <(curl -s https://codecov.io/bash) -f lcov.info;

cargo clippy