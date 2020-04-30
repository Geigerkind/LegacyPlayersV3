echo ${GITHUB_TOKEN}
git clone --single-branch --branch ${BRANCH_NAME} https://Geigerkind:${GITHUB_TOKEN}@github.com/Geigerkind/LegacyPlayersV3
cd LegacyPlayersV3/Backend
cp .env_ci .env
cargo build
cargo test --features "integration"
cargo clippy