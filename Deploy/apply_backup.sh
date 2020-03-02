BACKUP_FILE=${1.zip}
TIMESTAMP=$(date +%s)

systemctl stop backend
systemctl stop mysqld

echo "Applying backup ${BACKUP_FILE}"

if [ -f "/root/DB_BACKUPS/${BACKUP_FILE}.zip"]; then
  unzip /root/DB_BACKUPS/${BACKUP_FILE}.zip

  UNPACKED_BACKUP_PATH="/root/root/DB_BACKUPS/${BACKUP_FILE}"
  if [ -d "${UNPACKED_BACKUP_PATH}"]; then
    mariabackup --prepare --target-dir=${UNPACKED_BACKUP_PATH}
    mv /var/lib/mysql ./mysql_${TIMESTAMP}
    mariabackup --copy-back --target-dir=${UNPACKED_BACKUP_PATH}
    rm -r /root/root
  fi
fi

echo "Done!"

systemctl start mysqld
systemctl start backend