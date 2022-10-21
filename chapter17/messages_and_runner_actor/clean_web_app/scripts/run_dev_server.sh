#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ..

export SECRET_KEY="secret"
export EXPIRE_MINUTES=60
export DB_URL="postgres://username:password@localhost:5433/to_do"
export REDIS_URL="redis://127.0.0.1/"
cargo run
