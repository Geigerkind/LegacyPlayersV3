FROM mariadb:10.4.8

COPY my.cnf /etc/mysql/
COPY docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh
RUN mkdir /initFiles
RUN chmod -R 777 /initFiles
RUN chmod +x /usr/local/bin/docker-entrypoint.sh
RUN apt update && apt install unzip
ENTRYPOINT ["docker-entrypoint.sh"]
CMD ["mysqld"]