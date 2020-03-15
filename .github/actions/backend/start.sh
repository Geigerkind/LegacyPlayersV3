echo ${GITHUB_TOKEN}
git clone https://Geigerkind:${GITHUB_TOKEN}@github.com/Geigerkind/LegacyPlayersV3
cd LegacyPlayersV3/Backend
cp .env_ci .env
cargo build
cargo test
cargo clippy