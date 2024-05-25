#!/usr/bin/env bash

set -e -m

# docker build --platform=linux/amd64 \
#   -t kube-registry:5000/eds-game-for-ftp-game-jam-2022-xvfb:latest \
#   -f xvfb/Dockerfile \
#   . &
# docker build --platform=linux/amd64 \
#   -t kube-registry:5000/eds-game-for-ftp-game-jam-2022-server:latest \
#   -f server/Dockerfile \
#   . &
docker build --platform=linux/amd64 \
  -t kube-registry:5000/eds-game-for-ftp-game-jam-2022-client:latest \
  -f client/Dockerfile \
  . &
# docker build --platform=linux/amd64 \
#   -t kube-registry:5000/eds-game-for-ftp-game-jam-2022-proxy:latest \
#   -f proxy/Dockerfile \
#   . &

wait

# docker push --platform=linux/amd64 kube-registry:5000/eds-game-for-ftp-game-jam-2022-xvfb:latest &
# docker push --platform=linux/amd64 kube-registry:5000/eds-game-for-ftp-game-jam-2022-server:latest &
docker push --platform=linux/amd64 kube-registry:5000/eds-game-for-ftp-game-jam-2022-client:latest &
# docker push --platform=linux/amd64 kube-registry:5000/eds-game-for-ftp-game-jam-2022-proxy:latest &

wait
