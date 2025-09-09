use std::io::Result;

use camino::Utf8Path;
use ignore::gitignore::Gitignore;
use passwdmap::SysMap;

mod repo_dir;
mod vcs;

pub use repo_dir::RepoDir;
pub use vcs::Vcs;

#[derive(Debug)]
pub struct SharedInfo {
    pub root: RepoDir,
    pub sysmap: SysMap,
    pub vcs: Vcs,
    pub ignore: Gitignore,
}

impl SharedInfo {
    pub fn load(root: Option<Box<Utf8Path>>, vcs: Option<Vcs>) -> Result<Self> {
        let root = root.map(RepoDir::new).unwrap_or_default();
        let sysmap = SysMap::load()?;
        let vcs = vcs
            .or_else(|| Vcs::detect_vcs(&root))
            .expect("unable to detect vcs");
        let ignore_file = root.ignorefile_path(&vcs);
        assert_eq!(&vcs, &Vcs::Git);
        let (ignore, e) = Gitignore::new(ignore_file);
        if let Some(e) = e {
            panic!("{:?}", e);
        };
        Ok(Self {
            root,
            sysmap,
            vcs,
            ignore,
        })
    }
    pub fn uid_to_user(&self, uid: u32) -> Option<&str> {
        self.sysmap.user_maps.id_to_name.get(uid).map(|e| &**e)
    }
    pub fn gid_to_group(&self, gid: u32) -> Option<&str> {
        self.sysmap.group_maps.id_to_name.get(gid).map(|e| &**e)
    }
    pub fn user_to_uid(&self, uid: impl AsRef<str>) -> Option<u32> {
        self.sysmap.user_maps.name_to_id.get(uid.as_ref()).copied()
    }
    pub fn group_to_gid(&self, gid: impl AsRef<str>) -> Option<u32> {
        self.sysmap.group_maps.name_to_id.get(gid.as_ref()).copied()
    }
}
