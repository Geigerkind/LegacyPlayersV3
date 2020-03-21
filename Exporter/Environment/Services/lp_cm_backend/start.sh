source ./configuration.sh
cd /Backend
# DEBUG REMOVE uncomment later
#cargo clean
sleep 10s
RUST_ENV=production cargo run --release