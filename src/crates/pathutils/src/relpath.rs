use std::{env::current_dir, iter::repeat_n, path::Component};

use camino::Utf8Component;

use crate::{
    abspath::{abspath, abspath_utf8},
    std_path_as_path::*,
};

fn common_components<'a>(
    path_list: &'a Path,
    start_list: &Path,
) -> impl Iterator<Item = Component<'a>> {
    path_list
        .components()
        .zip(start_list.components())
        .take_while(|(a, b)| a == b)
        .map(|(a, _)| a)
}

pub fn relpath(path: &Path, start: &Path) -> PathBuf {
    let path = abspath(path);
    let start = abspath(start);

    let start_comp_count = start.components().count();

    let common_count = common_components(&path, &start).count();

    let a = repeat_n(Component::ParentDir, start_comp_count - common_count);
    let b = path.components().skip(common_count);

    PathBuf::from_iter(a.chain(b))
}

pub fn relpath_cwd(path: &Path) -> PathBuf {
    relpath(path, &current_dir().unwrap())
}

fn common_components_utf8<'a>(
    path_list: &'a Utf8Path,
    start_list: &Utf8Path,
) -> impl Iterator<Item = Utf8Component<'a>> {
    path_list
        .components()
        .zip(start_list.components())
        .take_while(|(a, b)| a == b)
        .map(|(a, _)| a)
}

fn path_buf_from_components<P: AsRef<Utf8Path>, I: IntoIterator<Item = P>>(iter: I) -> Utf8PathBuf {
    let mut res = Utf8PathBuf::from_iter(iter);
    if res.components().count() == 0 {
        res.push(".");
    }
    res
}

pub fn relpath_utf8(path: impl AsRef<Utf8Path>, start: impl AsRef<Utf8Path>) -> Utf8PathBuf {
    let path = abspath_utf8(path.as_ref());
    let start = abspath_utf8(start.as_ref());

    let start_comp_count = start.components().count();

    let common_count = common_components_utf8(&path, &start).count();

    let a = repeat_n(Utf8Component::ParentDir, start_comp_count - common_count);
    let b = path.components().skip(common_count);

    path_buf_from_components(a.chain(b))
}

pub fn relpath_utf8_cwd(path: &Utf8Path) -> Utf8PathBuf {
    let cwd = Utf8PathBuf::from_path_buf(current_dir().unwrap()).unwrap();
    relpath_utf8(path, &cwd)
}

mod test {
    #[test]
    fn test_1() {
        use super::relpath_utf8;

        let res = relpath_utf8("/root/repo", "/root");
        assert_eq!(res, "repo");
    }
    #[test]
    fn test_2() {
        use super::relpath_utf8;

        let res = relpath_utf8("/abc/def/ghi", "/jkl");
        assert_eq!(res, "../abc/def/ghi");
    }
}
