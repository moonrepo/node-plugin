pub mod commands;
mod node_dist;
mod package_json;

pub use node_dist::*;
pub use package_json::*;

use proto_pdk_api::HostEnvironment;

pub fn get_globals_dirs(env: &HostEnvironment) -> Vec<String> {
    let mut dirs = vec![];

    // Windows for some reason removes the /bin suffix when installing into it,
    // so we also need to account for the path without /bin. But keep the /bin path
    // as the final path and for the install to trigger correctly.
    if env.os.is_windows() {
        dirs.push("$PROTO_HOME/tools/node/globals".into());
    }

    dirs.push("$PROTO_HOME/tools/node/globals/bin".into());
    dirs
}
