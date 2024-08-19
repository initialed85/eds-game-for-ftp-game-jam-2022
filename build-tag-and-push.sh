#!/usr/bin/env bash

set -e -m

if [[ "${1}" == "xvfb" ]] || [[ "${1}" == "" ]]; then
  docker build --progress plain --platform=linux/amd64 \
    -t kube-registry:5000/eds-game-for-ftp-game-jam-2022-xvfb:latest \
    -f xvfb/Dockerfile \
    .
  docker push kube-registry:5000/eds-game-for-ftp-game-jam-2022-xvfb:latest
fi

if [[ "${1}" == "server" ]] || [[ "${1}" == "" ]]; then
  docker build --progress plain --platform=linux/amd64 \
    -t kube-registry:5000/eds-game-for-ftp-game-jam-2022-server:latest \
    -f server/Dockerfile \
    .
  docker push kube-registry:5000/eds-game-for-ftp-game-jam-2022-server:latest
fi

if [[ "${1}" == "client" ]] || [[ "${1}" == "" ]]; then
  docker build --progress plain --platform=linux/amd64 \
    -t kube-registry:5000/eds-game-for-ftp-game-jam-2022-client:latest \
    -f client/Dockerfile \
    .
  docker push kube-registry:5000/eds-game-for-ftp-game-jam-2022-client:latest
fi

if [[ "${1}" == "proxy" ]] || [[ "${1}" == "" ]]; then
  docker build --progress plain --platform=linux/amd64 \
    -t kube-registry:5000/eds-game-for-ftp-game-jam-2022-proxy:latest \
    -f proxy/Dockerfile \
    .
  docker push kube-registry:5000/eds-game-for-ftp-game-jam-2022-proxy:latest
fi
