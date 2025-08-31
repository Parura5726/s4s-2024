# CLIC x S4S - 2024

## Dev

### Dependencies

You will need the following:

- [docker](https://www.docker.com/get-started/)
- A rust toolchain [(most likely cargo)](https://github.com/rust-lang/cargo)
- An operating system supporting UNIX sockets.

## Deploy

Run `docker build -t s4s-2025_frontend app` and `docker build -t s4s-2025_runner backend/deps`
(the name of the container can be set in `src/docker.rs`, and the front/backend server URLs in
`app/Dockerfile`).

You can now run `docker run -p 3000:3000 s4s-2025_frontend` to deploy the frontend.

To deploy the backend, run (in the `backend/` directory)
`DATA_DIR=/your/data/directory SOCK_DIR=/your/socket/directory cargo run`,
with DATA\_DIR and SOCK\_DIR set appropriately (and empty).

On its own, this will likely not work without a TLS stripper (due to Next.js making all its
requests by HTTPS), so an example [nginx](https://nginx.org), configuration is provided,
assuming an appropriate certificate is located at /etc/ssl/certs/selfsigned.crt.

If needed, a self-signed certificate can be generated using
`# openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout /etc/ssl/private/selfsigned.key -out /etc/ssl/certs/selfsigned.crt`

Finally, you can access the website on <https://localhost/s4s>.
