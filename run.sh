#!/bin/bash

if [[ ${1} == "" ]]
then
    echo "no day number provided"
    exit 1
fi
cargo run --bin "day${1}"
