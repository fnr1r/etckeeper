use std::io::Result;

use crate::cli::MetadataArgs;

use super::shared_info::SharedInfo;

fn metadata_apply(info: &SharedInfo) -> Result<()> {
    let mut input = info.root.metadata_read()?;
    // TODO
    Ok(())
}

fn metadata_save(info: &SharedInfo) -> Result<()> {
    let mut out = info.root.metadata_write()?;
    // TODO
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
