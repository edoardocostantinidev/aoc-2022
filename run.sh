#!/bin/bash

for d in */ ; do
    cd "$d" && cargo run && cd ..
done
