use proto_pdk_api::ExecCommandInput;
use std::path::Path;

pub fn install_global(dependency: &str, globals_dir: &Path) -> ExecCommandInput {
    let mut cmd = ExecCommandInput::inherit(
        "npm",
        [
            "install",
            "--global",
            "--loglevel",
            "warn",
            "--no-audit",
            "--no-update-notifier",
            dependency,
        ],
    );

    cmd.env_vars
        .insert("PROTO_INSTALL_GLOBAL".into(), "true".into());

    // Remove the /bin component
    cmd.env_vars.insert(
        "PREFIX".into(),
        globals_dir.parent().unwrap().to_string_lossy().to_string(),
    );

    cmd
}

pub fn uninstall_global(dependency: &str, globals_dir: &Path) -> ExecCommandInput {
    let mut cmd = ExecCommandInput::inherit(
        "npm",
        ["uninstall", "--global", "--loglevel", "warn", dependency],
    );

    cmd.env_vars
        .insert("PROTO_INSTALL_GLOBAL".into(), "true".into());

    // Remove the /bin component
    cmd.env_vars.insert(
        "PREFIX".into(),
        globals_dir.parent().unwrap().to_string_lossy().to_string(),
    );

    cmd
}
