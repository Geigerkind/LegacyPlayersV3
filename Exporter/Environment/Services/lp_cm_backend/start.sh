source ./configuration.sh
cd /Backend
# DEBUG REMOVE uncomment later
#cargo clean
RUST_ENV=production cargo run --release