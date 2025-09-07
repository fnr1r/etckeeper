use std::{borrow::Cow, env::current_dir};

use crate::{
    cleanpath::cleanpath,
    std_path_as_path::*,
    utils::{cow::cow_make_owned, utf8path::assert_utf8_cow_path},
};

pub fn abspath(path: &Path) -> Cow<'_, Path> {
    if path.is_absolute() {
        return Cow::Borrowed(path);
    }
    let path = current_dir().unwrap().join(path);
    cow_make_owned(cleanpath(&path))
}

pub fn abspath_utf8(path: &Utf8Path) -> Cow<'_, Utf8Path> {
    assert_utf8_cow_path(abspath(path.as_std_path()))
}
