# Changelog

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
