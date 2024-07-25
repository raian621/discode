#!/usr/bin/env bash

# Exports key value pairs in an env file into the environment variables of the
# current shell session.

if [ -z $1 ]; then
  echo "Usage: $0 <env file path>" 
  exit
fi

set -a            
source .env
set +a