use std::{
    env::current_dir,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Result, Write},
};

use camino::{Utf8Path, Utf8PathBuf};
use derive_more::AsRef;

use super::vcs::Vcs;

const ETCKEEPER_METADATA_PATH: &str = ".etckeeper";

#[derive(Debug, AsRef)]
pub struct RepoDir(pub Box<Utf8Path>);

impl RepoDir {
    pub fn new(path: Box<Utf8Path>) -> Self {
        Self(path)
    }
    pub fn metadata_path(&self) -> Utf8PathBuf {
        self.0.join(ETCKEEPER_METADATA_PATH)
    }
    pub fn metadata_read(&self) -> Result<impl BufRead> {
        Ok(BufReader::new(File::open(self.metadata_path())?))
    }
    pub fn metadata_write(&self) -> Result<impl Write> {
        Ok(BufWriter::new(File::create(self.metadata_path())?))
    }
    pub fn ignorefile_path(&self, vcs: &Vcs) -> Utf8PathBuf {
        let txt = vcs.as_str().to_lowercase();
        self.0.join(format!(".{}ignore", txt))
    }
}

impl Default for RepoDir {
    fn default() -> Self {
        let path = current_dir().unwrap();
        let path = Utf8PathBuf::from_path_buf(path).unwrap();
        Self::new(path.into_boxed_path())
    }
}
