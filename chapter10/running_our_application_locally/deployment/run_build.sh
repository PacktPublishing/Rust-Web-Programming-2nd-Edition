#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH


scp -i "~/.ssh/keys/remotebuild.pem" ./docker-compose.yml ec2-user@$1:/home/ec2-user/docker-compose.yml


echo "installing Rust"
ssh -i "~/.ssh/keys/remotebuild.pem" -t ec2-user@$1 << EOF
  until [ -f ./output.txt ]
  do
      sleep 2
  done
  echo "File found"
EOF
echo "Rust has been installed"


echo "logging in to Docker"
ssh -i "~/.ssh/keys/remotebuild.pem" -t ec2-user@$1 << EOF
  echo $3 | docker login --username $2 --password-stdin
  docker-compose up -d
EOF
echo "logged in to Docker"

