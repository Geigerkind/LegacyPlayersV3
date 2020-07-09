running=1

updateFiles() {
  # TODO only execute this if git pull has changes
  cd /LegacyPlayersV3
  git stash
  git pull
  cd /

  # Manual Backend update
  rm -r /lp_cm_backend/Backend/src
  rm /lp_cm_backend/Backend/Cargo.toml
  cp -r /LegacyPlayersV3/Exporter/Backend/src /lp_cm_backend/Backend/
  cp /LegacyPlayersV3/Exporter/Backend/Cargo.toml /lp_cm_backend/Backend/

  # Manual DB update
  rm /lp_cm_mariadb/Database/patches/*
  cp /LegacyPlayersV3/Exporter/Database/patches/* /lp_cm_mariadb/Database/patches/

  ls -l /lp_cm_mariadb/Database/patches/

  #yes | docker-compose rm --all
  docker-compose build --no-cache lpcmbackend
}

cleanup() {
    echo "Container stopped, performing cleanup..."
    docker-compose stop
    running=0
}
trap 'cleanup' SIGTERM SIGINT

updateFiles

echo "Starting the service"
docker-compose up

TIME_COUNTER=0
while true; do
  if [ "${running}" = "1" ]; then
    if [ ${TIME_COUNTER} -gt 86400 ]; then
      TIME_COUNTER=0
      echo "Updating the service"
      docker-compose stop

      updateFiles

      echo "Starting the service"
      docker-compose up -d --build
    fi
    sleep 1s
    TIME_COUNTER=$((TIME_COUNTER+1))
  else
    break
  fi
done