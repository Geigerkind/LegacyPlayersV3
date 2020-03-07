cleanup() {
    echo "Container stopped, performing cleanup..."
    docker-compose stop
}
trap 'cleanup' SIGTERM

docker-compose up