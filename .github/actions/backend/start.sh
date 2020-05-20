git clone --single-branch --branch ${BRANCH_NAME} https://github.com/${GITHUB_REPOSITORY}
cd LegacyPlayersV3/Backend
cp .env_ci .env

if [ ! -z "${CODECOV_TOKEN}" ]; then
  cargo install grcov
  export CARGO_INCREMENTAL=0
  export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
  export RUSTDOCFLAGS="-Cpanic=abort"
  cargo build;
  cargo test --features "integration";

  ~/.cargo/bin/grcov ./target/debug/ -s . -t lcov --llvm --branch --ignore-not-existing --ignore "/*" --ignore "*/tests/*" --ignore "*/dto/*" --ignore "*/domain_value/*" -o lcov.info;
  bash <(curl -s https://codecov.io/bash) -f lcov.info;
else
  cargo build;
  cargo test --features "integration";
fi

cargo clippy