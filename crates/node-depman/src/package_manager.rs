#![allow(dead_code)]

use extism_pdk::Error;
use proto_pdk::{get_plugin_id, HostEnvironment, UnresolvedVersionSpec, VirtualPath};
use std::fmt;

#[derive(PartialEq)]
pub enum PackageManager {
    Npm,
    Pnpm,
    Yarn,
}

impl PackageManager {
    pub fn detect() -> Result<PackageManager, Error> {
        let id = get_plugin_id()?;

        Ok(if id.to_lowercase().contains("yarn") {
            PackageManager::Yarn
        } else if id.to_lowercase().contains("pnpm") {
            PackageManager::Pnpm
        } else {
            PackageManager::Npm
        })
    }

    pub fn get_package_name(&self, version: impl AsRef<UnresolvedVersionSpec>) -> String {
        if self.is_yarn_berry(version.as_ref()) {
            "@yarnpkg/cli-dist".into()
        } else {
            self.to_string()
        }
    }

    pub fn is_yarn_classic(&self, version: impl AsRef<UnresolvedVersionSpec>) -> bool {
        matches!(self, PackageManager::Yarn)
            && match version.as_ref() {
                UnresolvedVersionSpec::Alias(alias) => alias == "legacy" || alias == "classic",
                UnresolvedVersionSpec::Version(ver) => ver.major == 1,
                UnresolvedVersionSpec::Req(req) => req.comparators.iter().any(|c| c.major == 1),
                _ => false,
            }
    }

    pub fn is_yarn_berry(&self, version: impl AsRef<UnresolvedVersionSpec>) -> bool {
        matches!(self, PackageManager::Yarn)
            && match version.as_ref() {
                UnresolvedVersionSpec::Alias(alias) => alias == "berry" || alias == "latest",
                UnresolvedVersionSpec::Version(ver) => ver.major > 1,
                UnresolvedVersionSpec::Req(req) => req.comparators.iter().any(|c| c.major > 1),
                _ => false,
            }
    }

    pub fn get_globals_dir_prefix(
        &self,
        env: &HostEnvironment,
        globals_dir: &VirtualPath,
    ) -> String {
        let prefix = globals_dir
            .real_path()
            .unwrap()
            .to_string_lossy()
            .to_string();

        // On Windows, globals will be installed into the prefix as-is,
        // so binaries will exist in the root of the prefix.
        if env.os.is_windows() {
            return prefix;
        }

        // On Unix, globals are nested within a /bin directory, and since our
        // fixed globals dir ends in /bin, we must remove it and set the prefix
        // to the parent directory. This way everything resolves correctly.
        prefix.replace("/bin", "")
    }
}

impl fmt::Display for PackageManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackageManager::Npm => write!(f, "npm"),
            PackageManager::Pnpm => write!(f, "pnpm"),
            PackageManager::Yarn => write!(f, "yarn"),
        }
    }
}
