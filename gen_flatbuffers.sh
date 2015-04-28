#!/bin/bash

BUFFERS=$(cat flatbuffers.list)

for fbs in ${BUFFERS}; do
    echo "Generating FlatBuffers from '${fbs}'"
    flatc -r -I src -o $(dirname ${fbs}) ${fbs}
done
