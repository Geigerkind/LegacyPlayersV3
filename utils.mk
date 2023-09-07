#!/bin/make
ifdef ENV_FILE
  include $(ENV_FILE)
else
  ifneq "$(wildcard ../.env)" ""
    include ../.env
    ENV_FILE:=../.env
  endif
  ifneq "$(wildcard .env)" ""
    include .env
    ENV_FILE:=.env
  endif
endif

# Load all variable names from the envrionmental file.
ifndef ENV_FILE
  $(error No '.env' file found or specified.)
endif
VARIABLES?=$(shell sed -rn 's/[[:space:]]*([A-Za-z_]+)[[:space:]]*=.*/\1/p' $(ENV_FILE))

# Export the variables for envsubst.
export $(VARIABLES)
