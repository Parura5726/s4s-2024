#!/bin/sh

# DEPRECATED, DO NOT USE FOR DOCKER DEPLOYMENTS

# Compile all images using podman/buildah from a single script, can then be pushed to a repo or copied directly
# We use buildah because it allows building an image with device passthrough (required for fuse-overlayfs)

# build frontend
docker build app --tag s4s-2025_frontend

# build backend
docker build backend --tag s4s-2025_backend

# build and archive runner
docker build backend/deps --tag s4s-2025_runner
mkdir runner
docker save s4s-2025_runner -o runner/s4s-2025_runner.tar

# copy runner to backend
ctnr=$(buildah from s4s-2025_backend:latest)
buildah run -v `pwd`/runner:/runner:z $ctnr podman load -i /runner/s4s-2025_runner.tar
buildah commit "$ctnr" s4s-2025_backend
buildah rm "$ctnr"

rm -rf runner
