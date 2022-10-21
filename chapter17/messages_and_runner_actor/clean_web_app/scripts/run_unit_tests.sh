#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ..

export SECRET_KEY="secret"
export EXPIRE_MINUTES=60
cargo test