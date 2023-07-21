use proto_pdk::*;
use proto_pdk_test_utils::{create_plugin, generate_download_install_tests};
use starbase_sandbox::create_empty_sandbox;

mod npm {
    use super::*;

    generate_download_install_tests!("npm-test", "9.0.0");
}

mod pnpm {
    use super::*;

    generate_download_install_tests!("pnpm-test", "8.0.0");
}

mod yarn {
    use super::*;

    generate_download_install_tests!("yarn-test", "1.22.0");
}
