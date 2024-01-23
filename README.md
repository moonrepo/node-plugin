# Node.js plugin

Node.js, npm, pnpm, and yarn WASM plugins for [proto](https://github.com/moonrepo/proto).

## Installation

```shell
proto install node
proto install npm
```

These plugins are built-in to proto, but if you want to override it with an explicit version, add the following to `.prototools`.

```toml
[plugins]
node = "source:https://github.com/moonrepo/node-plugin/releases/download/vX.Y.Z/node_plugin.wasm"
npm|pnpm|yarn = "source:https://github.com/moonrepo/node-plugin/releases/download/vX.Y.Z/node_depman_plugin.wasm"
```

## Configuration

All plugins can be configured with a `.prototools` file.

- `bundled-npm` (bool) - When `node` is installed, also install `npm` with the version of npm that came bundled with Node.js. Defaults to `false`.
- `intercept-globals` (bool) - When npm, pnpm, or yarn attempt to install a global package, intercept the call and fail with an error message encouraging the use of `proto install-global` instead. Defaults to `false`.

```toml
[tools.node]
bundled-npm = true
```

## Hooks

### Post-install

After installation and `bundled-npm = true`, the version of npm that came bundled with Node.js will also be installed. This functionality can be skipped by passing `--no-bundled-npm` during installation.

```shell
proto install node -- --no-bundled-npm
```

## Contributing

Build the plugins:

```shell
cargo build --target wasm32-wasi
```

Test the plugins by running `proto` commands.

```shell
proto install node-test
proto list-remote npm-test
```
