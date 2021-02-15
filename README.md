## actix-proxy
> Silly proxy experiment, enforces an allow list of cookies and headers

### Code

- Config files are located in [/config](/config). These files are parsed into structs present in the [settings file](./src/settings.rs).
- Proxy code itself is in the [main file](./src/main.rs). Allow list enforcement is in the `forward()` method.

### Development setup

- [Install Rust](https://www.rust-lang.org/tools/install)

- to run locally:
```
cargo run
```

You can run the little test target NodeJS HTTP server as well via:
```bash
node ./resources/server.js
```
This server runs on 8080

- run lint:
```bash
cargo clippy
```

- build for release:
```bash
cargo build --release
```

