#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

rm -rf ../web_app/target/

scp -i "~/.ssh/keys/remotebuild.pem" -r ../web_app ec2-user@$1:/home/ec2-user/web_app



ssh -i "~/.ssh/keys/remotebuild.pem" -t ec2-user@$1 << EOF
  until [ -f ./output.txt ]
  do
      sleep 2
  done
  echo "File found"
  curl https://sh.rustup.rs -sSf | bash -s -- -y
  source ~/.cargo/env
  cd web_app
  cargo build
EOF


rm -rf ../front_end/node_modules/

scp -i "~/.ssh/keys/remotebuild.pem" -r ../front_end ec2-user@$1:/home/ec2-user/front_end



ssh -i "~/.ssh/keys/remotebuild.pem" -t ec2-user@$1 << EOF
  curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.37.2/install.sh | bash
  . ~/.nvm/nvm.sh
  nvm install --lts
  cd front_end
  npm install 
EOF
