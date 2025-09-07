use anyhow::Result;

mod cli;

fn main() -> Result<()> {
    let args = cli::cli();
    let cli::Cli { subcommand } = args;
    let Some(subcommand) = subcommand else {
        println!("Hello, world!");
        return Ok(());
    };
    use cli::Command as E;
    match subcommand {
        E::Complete { shell, file } => cli::generate_completions(shell, file)?,
    }
    Ok(())
}
