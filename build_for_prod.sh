#!/usr/bin/env bash

set -e -m

docker build \
  -t kube-registry:5000/eds-game-for-ftp-game-jam-2022-xvfb:latest \
  -f xvfb/Dockerfile \
  . &
docker build \
  -t kube-registry:5000/eds-game-for-ftp-game-jam-2022-server:latest \
  -f server/Dockerfile \
  . &
docker build \
  -t kube-registry:5000/eds-game-for-ftp-game-jam-2022-client:latest \
  -f client/Dockerfile \
  . &
docker build \
  -t kube-registry:5000/eds-game-for-ftp-game-jam-2022-proxy:latest \
  -f proxy/Dockerfile \
  . &

wait

docker push kube-registry:5000/eds-game-for-ftp-game-jam-2022-xvfb:latest &
docker push kube-registry:5000/eds-game-for-ftp-game-jam-2022-server:latest &
docker push kube-registry:5000/eds-game-for-ftp-game-jam-2022-client:latest &
docker push kube-registry:5000/eds-game-for-ftp-game-jam-2022-proxy:latest &

wait
