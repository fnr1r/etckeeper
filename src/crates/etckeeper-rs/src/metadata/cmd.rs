use anyhow::Result;
use std::io::Write;

use crate::cli::MetadataArgs;

use super::{
    formats::original_v0::{meta_apply, meta_save},
    index::index_repo,
    shared_info::SharedInfo,
};

fn metadata_apply(info: &SharedInfo) -> Result<()> {
    let mut input = info.root.metadata_read()?;
    meta_apply(&mut input, info)?;
    Ok(())
}

fn metadata_save(info: &SharedInfo) -> Result<()> {
    let files = index_repo(info.root.as_ref(), info)?;
    let mut out = info.root.metadata_write()?;
    meta_save(&mut *out, info, &files)?;
    out.flush()?;
    Ok(())
}

pub fn cmd_metadata(args: MetadataArgs) -> Result<()> {
    let MetadataArgs {
        command,
        repo_dir,
        vcs,
    } = args;
    let info = SharedInfo::load(repo_dir, vcs)?;
    use crate::cli::MetadataCommand as E;
    match command {
        E::Apply => metadata_apply(&info),
        E::Save => metadata_save(&info),
    }
}
