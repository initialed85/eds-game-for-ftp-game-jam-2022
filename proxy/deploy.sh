#!/usr/bin/env bash

set -e -x

base="eds-game-for-ftp-game-jam-2022"
name="${base}-proxy"

OUTGOING_IP="$(python3 ../get_outgoing_ip.py | xargs)"

docker kill "${name}" >/dev/null 2>&1 || true
docker rm -f "${name}" >/dev/null 2>&1 || true

# docker run -d --restart=always -p 1334:1334 --name "${name}" "${name}"
docker run \
  --rm -it \
  -p 80:80 \
  --add-host "server:${SERVER_HOST:-${OUTGOING_IP?-failed to discover OUTGOING_IP}}" \
  --add-host "client:${CLIENT_HOST:-${OUTGOING_IP?-failed to discover OUTGOING_IP}}" \
  --name "${name}" \
  "${name}"
