#!/bin/sh
# Read the goose secret key from the Docker Swarm secret mount and export it
# as the env var goosed expects. Using a secret mount instead of -e means the
# key never appears in `docker inspect` or any image layer.
if [ -f /run/secrets/goose_secret_key ]; then
    GOOSE_SERVER__SECRET_KEY=$(cat /run/secrets/goose_secret_key)
    export GOOSE_SERVER__SECRET_KEY
fi

exec /usr/local/bin/goosed "$@"
