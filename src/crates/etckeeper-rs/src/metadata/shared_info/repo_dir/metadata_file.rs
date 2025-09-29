use std::{
    fs::{File, OpenOptions, remove_file, rename},
    io::{BufWriter, Result, Write},
    os::unix::fs::OpenOptionsExt,
};

use camino::Utf8PathBuf;
use derive_more::{Deref, DerefMut};

#[derive(Debug, Deref, DerefMut)]
pub struct MetadataFileWriter {
    backup_path: Option<Utf8PathBuf>,
    #[deref]
    #[deref_mut]
    inner: BufWriter<File>,
}

impl MetadataFileWriter {
    pub fn open(path: Utf8PathBuf) -> Result<Self> {
        let backup_path = if path.exists() {
            let backup_name = path.file_name().unwrap();
            let backup_name = format!("{}~", backup_name);
            let backup_path = path.with_file_name(backup_name);
            if backup_path.exists() {
                remove_file(&backup_path)?;
            }
            rename(&path, &backup_path)?;
            Some(backup_path)
        } else {
            None
        };
        let inner = OpenOptions::new()
            .write(true)
            .create_new(true)
            .truncate(true)
            .mode(0o700)
            .open(path)
            .map(BufWriter::new)?;
        Ok(Self { backup_path, inner })
    }
}

impl Drop for MetadataFileWriter {
    fn drop(&mut self) {
        self.inner.flush().expect("failed to flush metadata file");
        let Some(backup_path) = self.backup_path.take() else {
            return;
        };
        remove_file(backup_path).expect("failed to remove metadata file backup");
    }
}
