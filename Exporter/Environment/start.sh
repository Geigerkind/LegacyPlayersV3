running=1

cleanup() {
    echo "Container stopped, performing cleanup..."
    docker-compose stop
    running=0
}
trap 'cleanup' SIGTERM SIGINT

docker-compose up

while true; do
  if [ "${running}" = "1" ]; then
    sleep 1
  else
    break
  fi
done