> # Migrated to a new repository: https://github.com/moonrepo/tools

# Node.js ecosystem plugins

Node.js, npm, pnpm, and yarn WASM plugins for [proto](https://github.com/moonrepo/proto).

## Installation

```shell
proto install node
proto install npm|pnpm|yarn
```

These plugins are built-in to proto, but if you want to override it with an explicit version, add the following to `.prototools`.

```toml
[plugins]
node = "source:https://github.com/moonrepo/node-plugin/releases/download/vX.Y.Z/node_plugin.wasm"
npm|pnpm|yarn = "source:https://github.com/moonrepo/node-plugin/releases/download/vX.Y.Z/node_depman_plugin.wasm"
```

## Configuration

All plugins can be configured with a `.prototools` file.

### Node.js

- `bundled-npm` (bool) - When `node` is installed, also install `npm` with the version of npm that came bundled with Node.js. Defaults to `false`.
- `dist-url` (string) - The distribution URL to download Node.js archives from. Supports `{version}` and `{file}` tokens.

```toml
[tools.node]
bundled-npm = true
dist-url = "https://..."
```

### Package managers

- `shared-globals-dir` (bool) - EXPERIMENTAL: Global npm, pnpm, or yarn packages are installed to a shared location: `~/.proto/tools/node/globals`. Defaults to `false`.

```toml
[tools.npm]
shared-globals-dir = true

# [tools.pnpm]
# [tools.yarn]
```

> To execute the shared globals, you'll need to add `~/.proto/tools/node/globals/bin` to `PATH` in your shell.

## Hooks

### Node.js

#### Post-install

After Node.js is installed and `bundled-npm` is enabled, the version of npm that came bundled with Node.js will also be installed. This functionality can also be skipped by passing `--no-bundled-npm` during installation.

```shell
proto install node -- --no-bundled-npm
```

### Package managers

#### Pre-run

Before a npm/pnpm/yarn command is ran and `shared-globals-dir` is enabled, this hook will modify the arguments or environment variables of the command when installing/removing/etc a global package. Is a no-op for other commands.

npm and yarn will set the `PREFIX` environment variable, while pnpm will set `--global-dir` and `--global-bin-dir` arguments.

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
