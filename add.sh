#!/bin/bash -e

NAME="$1"

if [[ -z "${NAME}" ]]; then
	echo "No name provided"
	exit 1
fi

cp -r ./template ./${NAME}

cd ${NAME}
sed -i "s/template/${NAME}/" Cargo.toml
