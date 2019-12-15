echo ${GITHUB_TOKEN}
git clone https://Geigerkind:${GITHUB_TOKEN}@github.com/Geigerkind/Jaylapp
cd Jaylapp/Backend
cp .env_ci .env
cargo build
cargo test