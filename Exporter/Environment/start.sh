running=1

cleanup() {
    echo "Container stopped, performing cleanup..."
    docker-compose stop
    running=0
}
trap 'cleanup' SIGTERM SIGINT

cd ./LegacyPlayersV3
git stash
git pull
cd ./
echo "Starting the service"
docker-compose up -d --build

TIME_COUNTER=0
while true; do
  if [ "${running}" = "1" ]; then
    if [ ${TIME_COUNTER} -gt 86400 ]; then
      TIME_COUNTER=0
      echo "Updating the service"
      docker-compose stop

      cd ./LegacyPlayersV3
      git stash
      git pull
      cd ./

      echo "Starting the service"
      docker-compose up -d --build
    fi
    sleep 1s
    TIME_COUNTER=$((TIME_COUNTER+1))
  else
    break
  fi
done