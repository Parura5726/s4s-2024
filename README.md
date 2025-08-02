# CLIC x S4S - 2024

## Dev

### Dependencies

You will need the following dependencies:

- [podman](https://podman.io/) and [buildah](https://buildah.io) to build the container images
- [docker](https://www.docker.com/get-started/) (optional, podman works too if podman-compose and podman-docker are installed as well)

## Run locally

Edit the `app/Dockerfile` to set the host environment variables,
then run the `build-images.sh` script (requires podman and buildah installed).

You can now run `docker compose up`, but it is recommended to run a TLS stripper
(for example [nginx](https://nginx.org)) to support https.

Now, you can access the website on <http://localhost:3000/s4s>.
