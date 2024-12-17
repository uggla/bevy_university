#!/bin/bash

set -euo pipefail

if ! [[ -f /usr/bin/podman ]]; then
  echo "podman is not available."
  exit 1
fi

set +e
#shellcheck disable=SC2009
server=$(ps -ef | grep "[p]ython3 server.py")
set -e

if [[ -z "$server" ]]; then
  echo "Document is not served, please run uv run server.py."
  exit 1
fi

current_dir=$(pwd)
cd "$(dirname "$0")"
cd ../slides

podman run -ti --userns=keep-id --rm -v "$PWD":/data:Z -e "HOSTNAME=$HOSTNAME" -e "OUTPUT=bevy_university" localhost/build_pdf:latest

cd "$current_dir"
exit 0
