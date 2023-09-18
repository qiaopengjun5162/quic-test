# quic-test

```sh
2967  cargo new quic-test
2968  cd quic-test
2969  c
2970  mkdir examples
2971  mkdir fixtures
2972  touch fixtures/cert.pem
2973  touch fixtures/key.pem
2974  cargo add s2n-quic
2975  cargo add tokio --features full
2976  cargo add anyhow
2977  touch examples/client.rs
2978  touch examples/server.rs
2979  cargo run --example server
2980  cargo run --example client
2981* echo "# quic-test" >> README.md
```
