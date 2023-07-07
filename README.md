# Node.js plugin

Node.js, npm, pnpm, and yarn WASM plugins for [proto](https://github.com/moonrepo/proto).

## Contributing

Build the plugins:

```shell
cargo build --target wasm32-wasi
```

Test the plugins by running `proto` commands. Requires proto >= v0.12.

```shell
proto install node-test
proto list-remote npm-test
```
