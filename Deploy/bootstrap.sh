HOST_USER='root'
HOST_IP='51.38.125.33'
DB_PASSWORD=${1}

unzip ./Keys.zip

scp -r ./Keys ${HOST_USER}@${HOST_IP}:/${HOST_USER}/
scp -r ./init.sh ${HOST_USER}@${HOST_IP}:/${HOST_USER}/

ssh ${HOST_USER}@${HOST_IP} "bash init.sh ${DB_PASSWORD} && rm init.sh && rm ~/Keys/ovh.ini"
