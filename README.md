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
- `shared-globals-dir` (bool) - Installs npm, pnpm, or yarn global packages to a shared location: `~/.proto/tools/node/globals`. Defaults to `false`.

```toml
[tools.node]
bundled-npm = true
shared-globals-dir = true
```

## Hooks

### Post-install

After installation and `bundled-npm` is enabled, the version of npm that came bundled with Node.js will also be installed. This functionality can be skipped by passing `--no-bundled-npm` during installation.

```shell
proto install node -- --no-bundled-npm
```

### Pre-run

Before a npm/pnpm/yarn command is ran and `shared-globals-dir` is enabled, this hook will modify the arguments or environment variables of the command when installing a global package.

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
