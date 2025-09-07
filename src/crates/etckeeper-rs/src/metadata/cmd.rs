use std::io::Result;

use crate::cli::MetadataArgs;

fn metadata_apply() -> Result<()> {
    // TODO
    Ok(())
}

fn metadata_save() -> Result<()> {
    // TODO
    Ok(())
}

pub fn cmd_metadata(args: MetadataArgs) -> Result<()> {
    let MetadataArgs { command } = args;
    use crate::cli::MetadataCommand as E;
    match command {
        E::Apply => metadata_apply(),
        E::Save => metadata_save(),
    }
}
