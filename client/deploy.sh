#!/usr/bin/env bash

set -e -x

base="eds-game-for-ftp-game-jam-2022"
name="${base}-client"

docker kill "${name}" >/dev/null 2>&1 || true
docker rm -f "${name}" >/dev/null 2>&1 || true

# docker run -d --restart=always -p 1334:1334 --name "${name}" "${name}"
docker run \
  --rm -it \
  -p 1334:80 \
  --name "${name}" \
  "${name}"
