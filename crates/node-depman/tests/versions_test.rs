// use proto_pdk::*;
use proto_pdk_test_utils::{create_plugin, generate_resolve_versions_tests};
use starbase_sandbox::create_empty_sandbox;

mod npm {
    use super::*;

    generate_resolve_versions_tests!("npm-test", {
        "7" => "7.24.2",
        "8.1" => "8.1.4",
        "9.7.2" => "9.7.2",
    });
}

mod pnpm {
    use super::*;

    generate_resolve_versions_tests!("pnpm-test", {
        "7" => "7.33.5",
        "8.1" => "8.1.1",
        "dev" => "6.23.7-202112041634",
    });
}

mod yarn {
    use super::*;

    generate_resolve_versions_tests!("yarn-test", {
        "1" => "1.22.19",
        "2" => "2.4.2",
        "3" => "3.6.1",
        "berry" => "3.6.1",
    });
}
