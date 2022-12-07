#!/usr/bin/env bash

set -e -x

base="eds-game-for-ftp-game-jam-2022"
name="${base}-proxy"

docker build --progress plain -t "${name}" -f ./Dockerfile ../
