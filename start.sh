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
cargo run --release
done
