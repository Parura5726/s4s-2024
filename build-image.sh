#!/bin/sh

# build docker image to run submissions with buildah

ctnr=$(buildah from alpine:latest)
ctnr_name="clic-s4s-2025"

if [ ! -d "backend/deps" ]; then
    echo "Cannot find backend/deps, please run me from the root directory of the project"
    exit 1
fi

echo "Preparing container..."
buildah run $ctnr -- apk add openjdk21 python3 g++
echo "Copying dependency files..."
buildah copy $ctnr `pwd`/backend/deps /deps
echo "Commiting to $ctnr_name"
buildah commit "$ctnr" "$ctnr_name"
echo "Done!"
