include ../utils.mk
include ../.env

# Assume all available sites are enabled if NGINX_SITES_ENABLED is not defined.
NGINX_SITES_ENABLED?=$(notdir $(basename $(wildcard sites-available/*)))

_SITES_ENABLED?=$(foreach site,$(NGINX_SITES_ENABLED),sites-enabled/$(site))

all: clean $(_SITES_ENABLED) 

.PHONY : all

# Implicit envsubst target.
sites-enabled/%: sites-available/%.in $(ENV_FILE)
	envsubst '$(addprefix $$,$(VARIABLES))' < $< > $@

clean: 
	rm -rf sites-enabled 
	mkdir sites-enabled


