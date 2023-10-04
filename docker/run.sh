#!/bin/bash

if [ -z "${IMAGE}" ]; then
    echo "ERROR: The IMAGE environment variable is not set."
    exit 1
fi

if [ "${GITHUB_ACTIONS}" == "true" ]; then
    TTY_FLAG="-t"
elif tty -s; then
    TTY_FLAG="-ti"
else
    TTY_FLAG="-i"
fi

DIR="$( cd "$( dirname "$0" )" && pwd )"
PARENT_DIR="$(dirname "$DIR")"
MOUNT_CMD="-v ${PARENT_DIR}:/opt/noggin -v ${PARENT_DIR}/build-cache:/root/.cache"

ENV_VARS="-e CARGO_TARGET_DIR=/root/.cache/rust -e CARGO_HOME=/root/.cache/cargo"

docker run --rm ${TTY_FLAG} ${ENV_VARS} ${MOUNT_CMD} ${IMAGE} "$@"