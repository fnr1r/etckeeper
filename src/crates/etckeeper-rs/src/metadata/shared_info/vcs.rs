use camino::Utf8Path;
use strum::{EnumString, IntoStaticStr};

use super::repo_dir::RepoDir;

#[derive(Debug, Clone, PartialEq, Eq, EnumString, IntoStaticStr)]
pub enum Vcs {
    Git,
    Hg,
    Bzr,
    Darcs,
}

impl Vcs {
    pub fn as_str(&self) -> &'static str {
        self.into()
    }
    fn detect_vcs_from_env() -> Option<Self> {
        let Ok(v) = std::env::var("VCS") else {
            return None;
        };
        v.parse().ok()
    }
    fn detect_vcs_from_dir(path: &Utf8Path) -> Option<Self> {
        for (subdir, res) in [
            (".git", Self::Git),
            (".hg", Self::Hg),
            (".bzr", Self::Bzr),
            ("_darcs", Self::Darcs),
        ] {
            if !path.join(subdir).exists() {
                continue;
            }
            return Some(res);
        }
        None
    }
    pub fn detect_vcs(path: &RepoDir) -> Option<Self> {
        if let Some(vcs) = Self::detect_vcs_from_env() {
            return Some(vcs);
        };
        if let Some(vcs) = Self::detect_vcs_from_dir(path.as_ref()) {
            return Some(vcs);
        };
        None
    }
}
