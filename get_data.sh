#!/bin/bash

cookie=`cat cookie`
if [[ $# == 0 ]]
then
    echo "day number is not specified"
    exit 1
fi
curl -b "session=${cookie}" "https://adventofcode.com/2022/day/${1}/input" > "./data/day${1}.input"
