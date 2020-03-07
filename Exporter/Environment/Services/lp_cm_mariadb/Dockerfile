FROM mariadb:10.4.8

COPY my.cnf /etc/mysql/
COPY docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh
COPY ./cleanup.sh ./
COPY ./post_cleanup.sh ./
RUN chmod +x ./cleanup.sh
RUN chmod +x ./post_cleanup.sh

RUN ./cleanup.sh
ADD ./Database ./Database
RUN ls -l ./ && chmod -R 777 ./Database
RUN ./post_cleanup.sh

RUN chmod +x /usr/local/bin/docker-entrypoint.sh
RUN apt update && apt install unzip
ENTRYPOINT ["docker-entrypoint.sh"]
CMD ["mysqld"]