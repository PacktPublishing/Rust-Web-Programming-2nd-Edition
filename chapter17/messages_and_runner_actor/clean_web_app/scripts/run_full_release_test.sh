#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

if [ "$(uname -m)" = "arm64" ]
then 
    cp ../builds/arch_build ../Dockerfile
else 
    cp ../builds/server_build ../Dockerfile
fi

cd ../tests

# build the images and network
docker-compose build --no-cache
docker-compose up -d 

# wait until rust server is running
sleep 5

# run the api tests
newman run to_do_items.postman_collection.json

# destroy the container and image
docker-compose down
docker image rm test_server
docker image rm init_test_db
docker image rm test_postgres
rm ../Dockerfile
