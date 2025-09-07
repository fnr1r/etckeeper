use std::borrow::Cow;

use crate::{std_path_as_path::*, utils::utf8path::assert_utf8_cow_path};

pub fn cleanpath(path: &Path) -> Cow<'_, Path> {
    Cow::Owned(path_clean::clean(path))
}

pub fn cleanpath_utf8(path: &Utf8Path) -> Cow<'_, Utf8Path> {
    assert_utf8_cow_path(cleanpath(path.as_std_path()))
}
