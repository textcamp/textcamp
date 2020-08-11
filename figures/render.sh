#!/bin/sh

if ! command -v plantuml &> /dev/null
then
    echo "plantuml could not be found, please install."
    exit
fi

plantuml -tsvg *.puml