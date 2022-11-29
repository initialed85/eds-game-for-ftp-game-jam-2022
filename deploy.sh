#!/usr/bin/env bash

set -e

docker stop eds-game-for-ftp-game-jam-2022 || true
docker rm -f eds-game-for-ftp-game-jam-2022 || true

docker run -d --restart=always -p 1334:1334 eds-game-for-ftp-game-jam-2022
