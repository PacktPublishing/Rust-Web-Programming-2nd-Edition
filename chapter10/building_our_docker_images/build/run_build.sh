#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

rm -rf ../web_app/target/

ssh -i "~/.ssh/keys/remotebuild.pem" -t ec2-user@$1 "mkdir web_app"

scp -i "~/.ssh/keys/remotebuild.pem" -r ../web_app/src ec2-user@$1:/home/ec2-user/web_app/src
scp -i "~/.ssh/keys/remotebuild.pem" ../web_app/Cargo.toml ec2-user@$1:/home/ec2-user/web_app/Cargo.toml
scp -i "~/.ssh/keys/remotebuild.pem" ../web_app/config.yml ec2-user@$1:/home/ec2-user/web_app/config.yml
scp -i "~/.ssh/keys/remotebuild.pem" ../web_app/Dockerfile ec2-user@$1:/home/ec2-user/web_app/Dockerfile


echo "installing Rust"
ssh -i "~/.ssh/keys/remotebuild.pem" -t ec2-user@$1 << EOF
  curl https://sh.rustup.rs -sSf | bash -s -- -y
  until [ -f ./output.txt ]
  do
      sleep 2
  done
  echo "File found"
EOF
echo "Rust has been installed"


echo "building Rust Docker image"
ssh -i "~/.ssh/keys/remotebuild.pem" -t ec2-user@$1 << EOF
  cd web_app
  docker build . -t rust_app
EOF
echo "Docker image built"


echo "copying React app"
rm -rf ../front_end/node_modules/
scp -i "~/.ssh/keys/remotebuild.pem" -r ../front_end ec2-user@$1:/home/ec2-user/front_end
echo "React app copied"


echo "installing node on build server"
ssh -i "~/.ssh/keys/remotebuild.pem" -t ec2-user@$1 << EOF
  curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.37.2/install.sh | bash
  . ~/.nvm/nvm.sh
  nvm install --lts
EOF
echo "node installed"


echo "building front-end on server"
ssh -i "~/.ssh/keys/remotebuild.pem" -t ec2-user@$1 << EOF
  cd front_end
  docker build . -t front_end
EOF
echo "front-end Docker image has been built"
