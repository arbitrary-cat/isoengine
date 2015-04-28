#!/bin/bash

BUFFERS=$(echo \
    "src/grafix/anim/wire.fbs" \
    "src/entity/wire.fbs" \
)

for fbs in ${BUFFERS}; do
    echo "Generating FlatBuffers from '${fbs}'"
    flatc -r -I src -o $(dirname ${fbs}) ${fbs}
done
