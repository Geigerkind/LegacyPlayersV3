HOST_USER='root'
HOST_IP='51.38.99.189'
DB_PASSWORD=${1}

unzip ./Keys.zip

scp -R ./Keys ${HOST_USER}@${HOST_IP}:/${HOST_USER}/
scp -R ./init.sh ${HOST_USER}@${HOST_IP}:/${HOST_USER}/

ssh ${HOST_USER}@${HOST_IP} "bash init.sh ${DB_PASSWORD} && rm init.sh && rm ~/Keys/ovh.ini"