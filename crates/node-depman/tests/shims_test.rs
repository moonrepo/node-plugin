use proto_pdk_test_utils::*;

#[cfg(not(windows))]
mod npm {
    use super::*;

    generate_shims_test!("npm-test", ["npx", "node-gyp"]);
}

#[cfg(not(windows))]
mod pnpm {
    use super::*;

    generate_shims_test!("pnpm-test", ["pnpx"]);
}

#[cfg(not(windows))]
mod yarn {
    use super::*;

    generate_shims_test!("yarn-test", ["yarnpkg"]);
}
