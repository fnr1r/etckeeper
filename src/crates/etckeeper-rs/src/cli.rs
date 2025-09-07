use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand, ValueHint};
use clap_complete::{Shell, generate};
use std::{
    boxed::Box,
    fs::File,
    io::{Write, stdout},
    path::PathBuf,
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Print shell auto completions for the specified shell
    Complete {
        #[clap(value_enum)]
        /// The shell to generate auto completions for
        shell: Shell,
        #[clap(value_hint(ValueHint::FilePath))]
        /// The path to write the completions to
        file: Option<PathBuf>,
    },
}

pub fn cli() -> Cli {
    Cli::parse()
}

/// Creates completions for shells  
/// This function can also write them to a file
pub fn generate_completions(shell: Shell, file: Option<PathBuf>) -> Result<()> {
    eprintln!("Generating completions for {}.", shell);
    let mut buf: Box<dyn Write> = match file {
        Some(file) => {
            eprintln!("Writing completions to {}.", file.to_str().unwrap());
            Box::new(File::create(file)?)
        }
        None => Box::new(stdout()),
    };
    generate(shell, &mut Cli::command(), env!("CARGO_PKG_NAME"), &mut buf);
    Ok(())
}

#[test]
fn verify_cli() {
    Cli::command().debug_assert()
}
