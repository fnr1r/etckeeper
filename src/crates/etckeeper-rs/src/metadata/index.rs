use std::{io::Result as IoResult, rc::Rc};

use camino::{Utf8DirEntry, Utf8Path};
use pathutils::relpath_utf8;
use rustix::fs::FileType;

mod file;
mod xattrs;

pub use file::RepoFile;
pub use xattrs::FileXattrs;

use super::shared_info::SharedInfo;

const REPO_ROOT_IGNORE: phf::Set<&'static str> = phf::phf_set!(".git", ".hg", ".bzr", "_darcs",);

fn read_dir_sort(this: impl AsRef<Utf8Path>) -> IoResult<Vec<Utf8DirEntry>> {
    let mut res = this
        .as_ref()
        .read_dir_utf8()?
        .collect::<Result<Vec<_>, _>>()?;
    res.sort_by(|a, b| a.path().cmp(b.path()));
    Ok(res)
}

type VecRcRepoFileInfo = Vec<Rc<RepoFile>>;

#[inline]
fn index_handle_directory(res: &mut VecRcRepoFileInfo, info: &RepoFile) -> IoResult<()> {
    for entry in read_dir_sort(&info.path)? {
        index_handle_file(res, entry.path())?;
    }
    Ok(())
}

fn index_handle_file(res: &mut VecRcRepoFileInfo, path: &Utf8Path) -> IoResult<()> {
    let info = RepoFile::load_rc(path)?;
    res.push(info.clone());
    use FileType as E;
    match info.file_type() {
        E::RegularFile | E::Symlink => (),
        E::Directory => index_handle_directory(res, &info)?,
        e => todo!("{:?}", e),
    }
    Ok(())
}

/// We don't have to check other subdirs for vcs dirs, so we only do it for the root
#[inline]
fn index_handle_root_file(res: &mut VecRcRepoFileInfo, entry: Utf8DirEntry) -> IoResult<()> {
    let path = entry.file_name();
    if REPO_ROOT_IGNORE.contains(path) {
        return Ok(());
    }
    index_handle_file(res, entry.path())
}

#[inline]
pub fn index_repo_raw(root: &Utf8Path) -> IoResult<Vec<RepoFile>> {
    let mut res = vec![];
    let info = RepoFile::load(root)?;
    res.push(Rc::new(info));
    for entry in read_dir_sort(root)? {
        index_handle_root_file(&mut res, entry)?;
    }
    let res = res
        .into_iter()
        .map(|e| Rc::try_unwrap(e).unwrap())
        .collect::<Vec<_>>();
    Ok(res)
}

pub fn index_repo(root: &Utf8Path, info: &SharedInfo) -> IoResult<Vec<RepoFile>> {
    let mut res = index_repo_raw(root)?;
    for file in &mut res {
        file.path = relpath_utf8(&file.path, root);
    }
    // TODO: Move this into index_repo_raw
    res.retain(|e| {
        info.ignore
            .matched(&e.path, e.file_type() == FileType::Directory)
            .is_none()
    });
    res.sort_by(|a, b| a.path.as_str().cmp(b.path.as_str()));
    Ok(res)
}
