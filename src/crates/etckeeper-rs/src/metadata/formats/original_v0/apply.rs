use std::{
    borrow::Cow,
    fmt::Write,
    fs::create_dir_all,
    io::{BufRead, Error as IoError},
};

use anyhow::{Result, bail};
use camino::Utf8Path;
use easy_ext::ext;
use pathutils::utils::cow::cow_map;
use rustix::{
    fs::{Gid, Mode, Uid, chmod, chown},
    io::Errno,
};
use shellquote::singlequote_unescape;
use thiserror::Error;

use crate::metadata::shared_info::SharedInfo;

type AResult<'a, T, E = ApplyError<'a>> = core::result::Result<T, E>;

fn path_unescape(txt: &str) -> Cow<'_, Utf8Path> {
    let path = singlequote_unescape(txt).expect(txt);
    cow_map(path, |e| e.as_ref(), |e| e.into())
}

#[derive(Debug, Error)]
pub struct ApplyError<'a> {
    inner: IoError,
    operation: &'static str,
    path: &'a str,
}

impl std::fmt::Display for ApplyError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.operation)?;
        f.write_char(' ')?;
        f.write_str(self.path)?;
        f.write_str(": ")?;
        <IoError as std::fmt::Display>::fmt(&self.inner, f)?;
        Ok(())
    }
}

impl<'a> ApplyError<'a> {
    fn new(inner: IoError, operation: &'static str, path: &'a str) -> Self {
        Self {
            inner,
            operation,
            path,
        }
    }
}

trait WithPathExt {
    fn with_ctx<'a>(self, operation: &'static str, path: &'a str) -> ApplyError<'a>;
}

impl WithPathExt for IoError {
    fn with_ctx<'a>(self, operation: &'static str, path: &'a str) -> ApplyError<'a> {
        ApplyError::new(self, operation, path)
    }
}

impl WithPathExt for Errno {
    fn with_ctx<'a>(self, operation: &'static str, path: &'a str) -> ApplyError<'a> {
        IoError::from(self).with_ctx(operation, path)
    }
}

#[ext]
impl<T, E: WithPathExt> core::result::Result<T, E> {
    fn with_ctx<'a>(self, operation: &'static str, path: &'a str) -> AResult<'a, T> {
        self.map_err(|e| e.with_ctx(operation, path))
    }
}

fn meta_handle_mkdir<'a>(txt: &'a str) -> AResult<'a, ()> {
    let (_, txt) = txt.split_once(' ').unwrap();
    let path = path_unescape(txt);
    create_dir_all(path.as_std_path()).with_ctx("mkdir", txt)?;
    Ok(())
}

fn meta_handle_chmod<'a>(path: &Utf8Path, raw_path: &'a str, mode: &str) -> AResult<'a, ()> {
    let mode = u32::from_str_radix(mode, 8).unwrap();
    chmod(path.as_std_path(), Mode::from_raw_mode(mode)).with_ctx("chmod", raw_path)?;
    Ok(())
}

fn meta_handle_chown<'a>(
    path: &Utf8Path,
    raw_path: &'a str,
    owner: &str,
    info: &SharedInfo,
) -> AResult<'a, ()> {
    let owner = if let Some(owner) = singlequote_unescape(owner) {
        info.user_to_uid(owner)
    } else {
        owner.parse().ok()
    }
    .unwrap();
    let owner = Uid::from_raw(owner);
    chown(path.as_std_path(), Some(owner), None).with_ctx("chown", raw_path)?;
    Ok(())
}

fn meta_handle_chgrp<'a>(
    path: &Utf8Path,
    raw_path: &'a str,
    group: &str,
    info: &SharedInfo,
) -> AResult<'a, ()> {
    let group = if let Some(group) = singlequote_unescape(group) {
        info.group_to_gid(group)
    } else {
        group.parse().ok()
    }
    .unwrap();
    let group = Gid::from_raw(group);
    chown(path.as_std_path(), None, Some(group)).with_ctx("chgrp", raw_path)?;
    Ok(())
}

fn meta_handle_maybe<'a>(txt: &'a str, info: &SharedInfo) -> AResult<'a, ()> {
    let (op, txt) = txt.split_once(' ').unwrap();
    let (val, raw_path) = txt.split_once(' ').unwrap();
    let path = path_unescape(raw_path);
    match op {
        "chmod" => meta_handle_chmod(&path, raw_path, val)?,
        "chown" => meta_handle_chown(&path, raw_path, val, info)?,
        "chgrp" => meta_handle_chgrp(&path, raw_path, val, info)?,
        _ => panic!("bad sub-operation {}", op),
    }
    Ok(())
}

fn meta_handle_line<'a>(line: &'a str, info: &SharedInfo) -> AResult<'a, ()> {
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

pub fn meta_apply(input: &mut impl BufRead, info: &SharedInfo) -> Result<()> {
    let mut errors = vec![];
    for line in input.lines() {
        let line = line?;
        let Err(e) = meta_handle_line(&line, info) else {
            continue;
        };
        eprintln!("{}", &e);
        errors.push(e.inner);
    }
    if !errors.is_empty() {
        bail!("failed to apply some metadata");
    }
    Ok(())
}
