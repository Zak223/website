#!/bin/bash
clear
if [ -f .env ]
then
  export $(cat .env | sed 's/#.*//g' | xargs)
fi
export SECRET_KEY

rustup override set nightly
while true
do
git fetch; git pull
bash -c "exec -a thweb cargo run --release"
done
