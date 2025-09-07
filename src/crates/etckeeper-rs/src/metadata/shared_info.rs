use std::io::Result;

use camino::Utf8Path;

mod repo_dir;
mod vcs;

pub use repo_dir::RepoDir;
pub use vcs::Vcs;

#[derive(Debug)]
pub struct SharedInfo {
    pub root: RepoDir,
    pub vcs: Vcs,
}

impl SharedInfo {
    pub fn load(root: Option<Box<Utf8Path>>, vcs: Option<Vcs>) -> Result<Self> {
        let root = root.map(RepoDir::new).unwrap_or_default();
        let vcs = vcs
            .or_else(|| Vcs::detect_vcs(&root))
            .expect("unable to detect vcs");
        Ok(Self { root, vcs })
    }
}
