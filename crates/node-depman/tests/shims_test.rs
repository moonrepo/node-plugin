use proto_pdk_test_utils::{create_plugin, generate_global_shims_test};
use starbase_sandbox::{assert_snapshot, create_empty_sandbox};

#[cfg(not(windows))]
mod npm {
    use super::*;

    generate_global_shims_test!("npm-test", ["node-gyp"]);
}

#[cfg(not(windows))]
mod pnpm {
    use super::*;

    generate_global_shims_test!("pnpm-test", ["pnpx"]);
}

#[cfg(not(windows))]
mod yarn {
    use super::*;

    generate_global_shims_test!("yarn-test", ["yarnpkg"]);
}
