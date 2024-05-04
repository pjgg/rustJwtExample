# Rust JWT example

build: `cargo build`
run: `cargo run --release 1000000` 

You will need to load a private RSA key as an ENV, for example:

```
export JWT_SECRET="$(</home/pagonzal/Documents/workspace/jwtSignExample/src/main/resources/private_key.pem)"
```