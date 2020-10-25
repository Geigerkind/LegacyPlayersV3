MYSQL_UP=$(ps aux | grep mysql | wc -l)
if [ "${MYSQL_UP}" -le 1 ]; then
        exit
fi

DB_PASSWORD=$(cat /root/Keys/db_password)
TIMESTAMP=$(date +%s)

echo "Removing dumps older than 14 days..."
find /root/DB_BACKUPS/*.zip -mtime +14 -exec rm {} \;

echo "Dumping database..."
mariabackup --backup --target-dir /root/DB_BACKUPS/${TIMESTAMP} --user=root --password=${DB_PASSWORD}

echo "Zipping the dump..."
zip -r /root/DB_BACKUPS/${TIMESTAMP}.zip /root/DB_BACKUPS/${TIMESTAMP}

echo "Removing unzipped files..."
rm -rf /root/DB_BACKUPS/${TIMESTAMP}

echo "DONE!"