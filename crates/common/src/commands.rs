use proto_pdk_api::{ExecCommandInput, HostEnvironment, VirtualPath};

pub fn get_global_prefix(env: &HostEnvironment, globals_dir: &VirtualPath) -> String {
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

pub fn install_global(dependency: &str, globals_prefix: String) -> ExecCommandInput {
    let mut cmd = ExecCommandInput::inherit(
        "npm",
        [
            "install",
            dependency,
            "--global",
            "--loglevel",
            "warn",
            "--no-audit",
            "--no-update-notifier",
            "--prefix",
            &globals_prefix,
        ],
    );

    cmd.env.insert("PROTO_INSTALL_GLOBAL".into(), "true".into());

    cmd
}

pub fn uninstall_global(dependency: &str, globals_prefix: String) -> ExecCommandInput {
    let mut cmd = ExecCommandInput::inherit(
        "npm",
        [
            "uninstall",
            dependency,
            "--global",
            "--loglevel",
            "warn",
            "--prefix",
            &globals_prefix,
        ],
    );

    cmd.env.insert("PROTO_INSTALL_GLOBAL".into(), "true".into());

    cmd
}
