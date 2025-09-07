use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use camino::{Utf8Path, Utf8PathBuf};

use crate::utils::cow::cow_map;

pub fn assert_utf8_path(path: &Path) -> &Utf8Path {
    let path = path.as_os_str().as_encoded_bytes();
    let path = str::from_utf8(path).unwrap();
    Utf8Path::new(path)
}

pub fn assert_utf8_pathbuf(path: PathBuf) -> Utf8PathBuf {
    let path = path.into_os_string().into_encoded_bytes();
    let path = String::from_utf8(path).unwrap();
    Utf8PathBuf::from(path)
}

pub fn assert_utf8_cow_path(path: Cow<'_, Path>) -> Cow<'_, Utf8Path> {
    cow_map(path, assert_utf8_path, assert_utf8_pathbuf)
}
