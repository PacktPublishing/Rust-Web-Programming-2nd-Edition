#!/bin/bash

# move to directory of the project
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ..

# spin up docker and hold script until accepting connections
docker-compose up -d 
until pg_isready -h localhost -p 5433 -U username
do
  echo "Waiting for postgres"
  sleep 2;
done

cargo build
cargo test

# run server in background
cargo run config.yml &
SERVER_PID=$!
sleep 5

diesel migration run

cd scripts

# create the user
curl --location --request POST 'http://localhost:8000/v1/user/create' \
--header 'Content-Type: application/json' \
--data-raw '{
    "name": "maxwell",
    "email": "maxwellflitton@gmail.com",
    "password": "test"
}'

# login getting a fresh token
echo $(curl --location --request GET 'http://localhost:8000/v1/auth/login' \
--header 'Content-Type: application/json' \
--data-raw '{
    "username": "maxwell",
    "password": "test"
}') > ./fresh_token.json

#python3 ready_newman.py
TOKEN=$(jq '.token' fresh_token.json)
jq '.auth.apikey[0].value = '"$TOKEN"'' to_do_items.postman_collection.json > test_newman.json

newman run test_newman.json

rm ./test_newman.json
rm ./fresh_token.json

# shut down rust server
kill $SERVER_PID

cd ..
docker-compose down
