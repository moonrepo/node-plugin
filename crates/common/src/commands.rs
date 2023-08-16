use proto_pdk::ExecCommandInput;
use std::path::Path;

pub fn install_global(dependency: &str, globals_dir: &Path) -> ExecCommandInput {
    let mut cmd = ExecCommandInput::pipe(
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

    // Remove the /bin component
    cmd.env_vars.insert(
        "PREFIX".into(),
        globals_dir.parent().unwrap().to_string_lossy().to_string(),
    );

    cmd
}

pub fn uninstall_global(dependency: &str, globals_dir: &Path) -> ExecCommandInput {
    let mut cmd = ExecCommandInput::pipe(
        "npm",
        ["uninstall", "--global", "--loglevel", "warn", dependency],
    );

    // Remove the /bin component
    cmd.env_vars.insert(
        "PREFIX".into(),
        globals_dir.parent().unwrap().to_string_lossy().to_string(),
    );

    cmd
}
