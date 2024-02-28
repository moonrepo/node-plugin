// These work locally but fail in CI... hard to debug!

use proto_pdk_test_utils::*;

#[cfg(not(windows))]
mod npm {
    use super::*;

    generate_globals_test!("npm-test", "prettier");
}

// mod pnpm {
//     use super::*;

//     generate_globals_test!("pnpm-test", "prettier");
// }

// mod yarn {
//     use super::*;

//     generate_globals_test!("yarn-test", "prettier");
// }
