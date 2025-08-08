# CLIC x S4S - 2024

## Dev

### Dependencies

**WARNING: The backend uses rust feature lock_value_accessors which is currently unstable**

You will need the following dependencies:

- [buildah](https://buildah.io) to build the container images
- [docker](https://www.docker.com/get-started/) (optional, podman works too if podman-compose and podman-docker are installed as well)

## Deploy

Edit the `app/Dockerfile` to set the host environment variables,
then run the `build-images.sh` script (requires podman and buildah installed).

You can now run `docker compose up`, but it is highly recommended to run a TLS stripper
(for example [nginx](https://nginx.org)) to support https (an example configuration is provided).

Now, you can access the website on <https://localhost/s4s>.
