# Changelog

## 0.5.0

#### 💥 Breaking

- Updated the `npm` tool to create the `npx` shim instead of the `node` tool.
- Updated symlinked binaries to use the shell scripts instead of the source `.js` files (when applicable).

#### 🚀 Updates

- Updated to support proto v0.22 release.

#### ⚙️ Internal

- Updated dependencies.

## 0.4.3

#### 🐞 Fixes

- Temporarily fixed an issue where Yarn would fail to parse the npm registry response and error with "control character (\u0000-\u001F) found while parsing a string".

## 0.4.2

#### 🚀 Updates

- Support Yarn v4.

#### 🐞 Fixes

- Temporarily fixed an issue where calling `node` as a child process may fail.

## 0.4.1

#### 🐞 Fixes

- Potentially fixed a WASM memory issue.

## 0.4.0

#### 🚀 Updates

- Updated to support proto v0.20 release.

#### ⚙️ Internal

- Updated dependencies.

## 0.3.2

#### 🐞 Fixes

- Now strips the corepack hash from `packageManager` when parsing versions.

## 0.3.1

#### ⚙️ Internal

- Updated dependencies.

## 0.3.0

#### 🚀 Updates

- Added support for installing the canary release (when applicable).
- Brought back support for detecting a version from `package.json` engines.
- Updated to support proto v0.17 release.

## 0.2.1

#### 🚀 Updates

- Updated to support proto v0.16 release.

## 0.2.0

#### 🚀 Updates

- Added support for `install_global` and `uninstall_global`.
- Added `post_install` hook for installing the bundled npm.
- Updated to support proto v0.15 release.

#### 🐞 Fixes

- **npm**
  - Will no longer crash when parsing an invalid `package.json`.

## 0.1.0

#### 💥 Breaking

- Will no longer check `engines` in `package.json` when detecting a version.

#### 🚀 Updates

- Updated to support proto v0.14 release.

## 0.0.2

#### 🐞 Fixes

- **npm**
  - Improved version resolution for "bundled" alias.

## 0.0.1

#### 🎉 Release

- Initial release!
