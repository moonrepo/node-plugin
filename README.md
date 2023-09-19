# Node.js plugin

Node.js, npm, pnpm, and yarn WASM plugins for [proto](https://github.com/moonrepo/proto).

```shell
proto install node
```

## Post-install hook

After installation, the version of npm that came bundled with Node.js will also be installed. This
functionality can be skipped by passing `--no-bundled-npm` during installation.

```shell
proto install node -- --no-bundled-npm
```

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
