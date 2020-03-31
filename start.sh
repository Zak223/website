#!/bin/bash
clear
if [ -f .env ]
then
  export $(cat .env | sed 's/#.*//g' | xargs)
fi
export SECRET_KEY

rustup override set nightly
cargo run --release
