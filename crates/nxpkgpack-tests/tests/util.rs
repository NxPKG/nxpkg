use std::path::PathBuf;

use dunce::canonicalize;
use once_cell::sync::Lazy;

/// The nxpkg repo root. Should be used as the root when building with nxpkgpack
/// against fixtures in this crate.
pub static REPO_ROOT: Lazy<String> = Lazy::new(|| {
    let package_root = PathBuf::from(env!("NXPKG_PNPM_WORKSPACE_DIR"));
    canonicalize(package_root)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
});
