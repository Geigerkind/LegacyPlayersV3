source ./configuration.sh

cd /Backend
cargo clean
RUST_ENV=production cargo run --release