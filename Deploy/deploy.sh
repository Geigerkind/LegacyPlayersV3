REPOSITORY_NAME='Jaylapp'
HOST_USER='root'
BACKEND_USER='yajla'
HOST_IP='51.38.114.9'
NUM_CORES=$(nproc)
DB_PASSWORD=$(cat /root/Keys/db_password)

function cleanAssetCache {
  cd /root/cache/assets/
  for filename in $(find . -name "*.png") $(find . -name "*.jpg") $(find . -name "*.jpeg"); do
    if [ ! -f "${filename}" ]; then
      continue
    fi

    FILENAME_WITHOUT_PREFIX=${filename:2}

    if [[ ! -f "/root/${REPOSITORY_NAME}/Webclient/src/assets/${FILENAME_WITHOUT_PREFIX}" ]]; then
      rm ${FILENAME_WITHOUT_PREFIX}
      rm ${FILENAME_WITHOUT_PREFIX}.webp &> /dev/null # Ignore error if it had been deleted already
    fi
    DIR=$(dirname "${FILENAME_WITHOUT_PREFIX}")
    if [ -z "$(ls -A ${DIR})" ]; then
      rm -rf ${DIR}
    fi
  done
  cd /root/
}
function optimizeJpg {
  cd /root/${REPOSITORY_NAME}/Webclient/src/assets/
  MEDIA_DIR='/root/cache/assets/'
  for filename in $(find . -name "*.jpg") $(find . -name "*.jpeg"); do
    if [ ! -f "${filename}" ]; then
      continue
    fi

    while [ $(pgrep -c -P$$) -gt ${NUM_CORES} ]; do
        sleep 0.5;
    done

    BASEFILENAME=$(basename "${filename}");
    PATHTOFILE=$(dirname "${filename:2}");
    TARGETDIR="${MEDIA_DIR}${PATHTOFILE}";

    if [ ! -d "${TARGETDIR}" ]; then
        mkdir -p ${TARGETDIR};
    fi

    guetzli --quality 84 --nomemlimit ${filename} ${TARGETDIR}/${BASEFILENAME} > /dev/null 2> /dev/null &
  done
  cd /root
}
function optimizePng {
  cd /root/${REPOSITORY_NAME}/Webclient/src/assets/
  MEDIA_DIR='/root/cache/assets/'
  for filename in $(find . -name "*.png"); do
    if [ ! -f "${filename}" ]; then
      continue
    fi

    while [ $(pgrep -c -P$$) -gt ${NUM_CORES} ]; do
        sleep 0.5;
    done

    BASEFILENAME=$(basename "${filename}");
    PATHTOFILE=$(dirname "${filename:2}");
    TARGETDIR="${MEDIA_DIR}${PATHTOFILE}";

    if [ ! -d "${TARGETDIR}" ]; then
        mkdir -p ${TARGETDIR};
    fi

    zopflipng --iterations=5 --filters=2me --lossy_8bit --lossy_transparent -y ${filename} ${TARGETDIR}/${BASEFILENAME} > /dev/null 2> /dev/null &
  done
  cd /root
}
function convertToWebp {
  for filename in $(find /root/cache/assets/ -name "*.png") $(find /root/cache/assets/ -name "*.jpg") $(find /root/cache/assets/ -name "*.jpeg"); do
    if [ ! -f "${filename}" ]; then
      continue
    fi

    while [ $(pgrep -c -P$$) -gt ${NUM_CORES} ]; do
        sleep 0.5;
    done
    cwebp -q 30 ${filename} -o ${filename}.webp > /dev/null 2> /dev/null &
  done
}
function optimizeAssets {
  echo "Optimizing assets"
  mkdir -p /root/cache/assets &> /dev/null
  cleanAssetCache
  optimizeJpg
  optimizePng
  convertToWebp
}

function deployDatabase {
  echo "Deploying database"
  cd /root/${REPOSITORY_NAME}/Database
  bash merger.sh
  if [ -f "./merge.sql" ]; then
    systemctl start mysqld
    mysql -uroot -p${DB_PASSWORD} < merge.sql
    systemctl stop mysqld
    rm merge.sql
  fi
  cd /root
}

function deployWebclient {
  echo "Deploying webclient"
  cd /root/${REPOSITORY_NAME}/Webclient
  rm -rf /root/${REPOSITORY_NAME}/Webclient/node_modules
  rm /root/${REPOSITORY_NAME}/Webclient/package-lock.json
  npm install
  npm run-script build
  rm -rf /var/www/html/*
  cp -r /root/${REPOSITORY_NAME}/Webclient/dist/Webclient/* /var/www/html/
  cd /root

  # Deploying optimized assets
  cp -r /root/cache/assets/* /var/www/html/assets/
}

function deployBackend {
  echo "Deploying backend"
  cd /root/${REPOSITORY_NAME}/Backend
  rustup toolchain install nightly
  cargo update
  cargo build --release --all-features --jobs ${NUM_CORES}
  cargo install --path ./ --force
  cp /root/.cargo/bin/backend /home/${BACKEND_USER}/
  cp .env_prod /home/${BACKEND_USER}/.env
  cd /root
}

function updateConfigs {
  # Postfix
  cp /root/${REPOSITORY_NAME}/Deploy/conf/virtual /etc/postfix/
  cp /root/${REPOSITORY_NAME}/Deploy/conf/main.cf /etc/postfix/
  postmap /etc/postfix/virtual

  # Mariadb
  cp /root/${REPOSITORY_NAME}/Deploy/conf/my.conf /etc/

  # Nginx
  cp /root/${REPOSITORY_NAME}/Deploy/conf/nginx.conf /etc/nginx/
}

function stopServices {
  echo "Stopping services"
  systemctl stop nginx
  systemctl stop mysqld
  systemctl stop postfix
  systemctl stop backend
}

function startServices {
  echo "Starting services"
  systemctl start nginx
  systemctl start mysqld
  systemctl start postfix
  systemctl start backend
}

function waitForJobs {
  while [ $(jobs | grep Running | wc -l) -gt 0 ]; do
    sleep 0.5s
  done
}

function deploy {
  pacman -Syu --noconfirm

  cd /root/${REPOSITORY_NAME}
  git stash
  git pull
  cd /root

  optimizeAssets

  stopServices
  certbot renew

  deployDatabase
  deployWebclient
  deployBackend
  updateConfigs
  waitForJobs

  startServices
}

deploy