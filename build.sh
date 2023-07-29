#!/bin/sh

cd server
cargo build --release
cd ..
cp ./target/release/sakugahd-server .

cd client
yarn build
cp -r dist ../
