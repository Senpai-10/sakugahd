#!/bin/sh

set -e

cd server
cargo build --release
cd ..
cp ./target/release/sakugahd-server ~/.local/bin/

cd client
yarn build
sudo cp -Tr dist /usr/src/sakugahd-dist/
