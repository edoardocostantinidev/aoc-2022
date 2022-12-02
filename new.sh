#!/bin/bash

cargo new "day-$1"

cd "day-$1" || exit

aoc r
aoc d

cd ..
