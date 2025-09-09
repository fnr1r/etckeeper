use std::{
    borrow::Cow,
    fs::create_dir_all,
    io::{BufRead, Result as IoResult},
};

use camino::Utf8Path;
use pathutils::utils::cow::cow_map;
use rustix::fs::{Gid, Mode, Uid, chmod, chown};
use shellquote::singlequote_unescape;

use crate::metadata::shared_info::SharedInfo;

fn path_unescape(txt: &str) -> Cow<'_, Utf8Path> {
    let path = singlequote_unescape(txt).expect(txt);
    cow_map(path, |e| e.as_ref(), |e| e.into())
}

fn meta_handle_mkdir(txt: &str) -> IoResult<()> {
    let (_, txt) = txt.split_once(' ').unwrap();
    let path = path_unescape(txt);
    create_dir_all(path.as_std_path())?;
    Ok(())
}

fn meta_handle_chmod(path: &Utf8Path, mode: &str) -> IoResult<()> {
    let mode = u32::from_str_radix(mode, 8).unwrap();
    chmod(path.as_std_path(), Mode::from_raw_mode(mode))?;
    Ok(())
}

fn meta_handle_chown(path: &Utf8Path, owner: &str, info: &SharedInfo) -> IoResult<()> {
    let owner = if let Some(owner) = singlequote_unescape(owner) {
        info.user_to_uid(owner)
    } else {
        owner.parse().ok()
    }
    .unwrap();
    let owner = Uid::from_raw(owner);
    chown(path.as_std_path(), Some(owner), None)?;
    Ok(())
}

fn meta_handle_chgrp(path: &Utf8Path, group: &str, info: &SharedInfo) -> IoResult<()> {
    let group = if let Some(group) = singlequote_unescape(group) {
        info.group_to_gid(group)
    } else {
        group.parse().ok()
    }
    .unwrap();
    let group = Gid::from_raw(group);
    chown(path.as_std_path(), None, Some(group))?;
    Ok(())
}

fn meta_handle_maybe(txt: &str, info: &SharedInfo) -> IoResult<()> {
    let (op, txt) = txt.split_once(' ').unwrap();
    let (val, path) = txt.split_once(' ').unwrap();
    let path = path_unescape(path);
    match op {
        "chmod" => meta_handle_chmod(&path, val)?,
        "chown" => meta_handle_chown(&path, val, info)?,
        "chgrp" => meta_handle_chgrp(&path, val, info)?,
        _ => panic!("bad sub-operation {}", op),
    }
    Ok(())
}

fn meta_handle_line(line: &str, info: &SharedInfo) -> IoResult<()> {
    if line.is_empty() || line.starts_with('#') {
        return Ok(());
    }
    let (op, txt) = line.split_once(' ').unwrap();
    match op {
        "mkdir" => meta_handle_mkdir(txt)?,
        "maybe" => meta_handle_maybe(txt, info)?,
        _ => panic!("bad operation {}", op),
    }
    Ok(())
}

pub fn meta_apply(input: &mut impl BufRead, info: &SharedInfo) -> IoResult<()> {
    for line in input.lines() {
        meta_handle_line(&line?, info)?;
    }
    Ok(())
}
