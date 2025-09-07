use std::{io::Result as IoResult, rc::Rc};

use camino::{Utf8Path, Utf8PathBuf};
use rustix::fs::{FileType, Mode, Stat, lstat};

use super::FileXattrs;

#[derive(Debug)]
pub struct RepoFile {
    pub path: Utf8PathBuf,
    pub stat: Stat,
    pub xattrs: FileXattrs,
}

impl RepoFile {
    pub fn load(path: impl AsRef<Utf8Path>) -> IoResult<Self> {
        let path = path.as_ref().to_path_buf();
        let stat = lstat(path.as_std_path())?;
        let xattrs = FileXattrs::load(&path)?;
        Ok(Self { path, stat, xattrs })
    }
    pub fn load_rc(path: impl AsRef<Utf8Path>) -> IoResult<Rc<Self>> {
        Self::load(path).map(Rc::new)
    }
    pub const fn file_type(&self) -> FileType {
        FileType::from_raw_mode(self.stat.st_mode)
    }
    pub const fn uid(&self) -> u32 {
        self.stat.st_uid
    }
    pub const fn gid(&self) -> u32 {
        self.stat.st_gid
    }
    pub const fn mode(&self) -> Mode {
        Mode::from_raw_mode(self.stat.st_mode)
    }
}
