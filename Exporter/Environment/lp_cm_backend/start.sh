cd LegacyPlayersV3
git pull
cd Exporter/Backend
cargo clean
RUST_ENV=production cargo run --release