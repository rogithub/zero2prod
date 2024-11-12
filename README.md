# Comands

## Database migrations
``` bash
SKIP_PODMAN=true ./scripts/init_db.sh
```

``` bash
curl -v http://127.0.0.1/health_check
curl -i -X POST -d 'email=thomas_man@hotname.com&name=tom' http://127.0.0.1:8000/subscriptions/
```

``` bash
RUST_LOG=trace cargo run
```

We are using the 'bunyan' CLI to prettify the outputted logs 
The original 'bunyan' requieres NPM, but you can install a Rust-port with
`cargo install bunyan`
``` bash
TEST_LOG=true cargo test health_check_works | bunyan
```

## Podman
``` bash
podman build --tag zero2prod --file Dockerfile .
podman run -p 8000:8000 zero2prod
```