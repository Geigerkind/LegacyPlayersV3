include .env

.PHONY: up login.% restart.% nginx clean.% 

up: nginx
	docker-compose --env-file ./.env up --build

down: 
	docker-compose down

update:
	docker-compose down
	git pull
	cd nginx && make all && cd ..

configure:
	cd nginx && make all && cd ..

nginx:
	$(MAKE) -C $@ ENV_FILE=$(realpath .env)

# Login targets
login.webclient:
login.nginx:
login.%:
	docker exec -ti $* /bin/bash

# Restart targets
restart.webclient:
restart.nginx:
restart.%:
	docker-compose restart $*
