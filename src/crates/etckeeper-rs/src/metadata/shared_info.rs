use std::io::Result;

use camino::Utf8Path;
use ignore::gitignore::Gitignore;

mod repo_dir;
mod vcs;

pub use repo_dir::RepoDir;
pub use vcs::Vcs;

#[derive(Debug)]
pub struct SharedInfo {
    pub root: RepoDir,
    pub vcs: Vcs,
    pub ignore: Gitignore,
}

impl SharedInfo {
    pub fn load(root: Option<Box<Utf8Path>>, vcs: Option<Vcs>) -> Result<Self> {
        let root = root.map(RepoDir::new).unwrap_or_default();
        let vcs = vcs
            .or_else(|| Vcs::detect_vcs(&root))
            .expect("unable to detect vcs");
        let ignore_file = root.ignorefile_path(&vcs);
        assert_eq!(&vcs, &Vcs::Git);
        let (ignore, e) = Gitignore::new(ignore_file);
        if let Some(e) = e {
            panic!("{:?}", e);
        };
        Ok(Self { root, vcs, ignore })
    }
}
