#!/bin/bash
cargo build
cd ./target/debug/
sudo cp ./find-movie-cli-tool /bin
