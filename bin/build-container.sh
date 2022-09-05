#!/bin/bash

pushd "$(git rev-parse --show-toplevel)"

source bin/lib.sh

docker build --build-arg version=${version} --tag ${container}:${tag} .
docker tag ${container}:${tag} ${container}:${version}

popd > /dev/null
