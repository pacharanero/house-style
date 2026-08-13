// This example will receive the repository's permissive code licence before release.

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "example")]
#[command(about = "Example CLI following house style")]
#[command(version)]
pub struct Cli {
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List available commands.
    List,
    /// Print version information.
    Version,
}

pub fn run() -> Result<()> {
    run_with(Cli::parse())
}

pub fn run_with(cli: Cli) -> Result<()> {
    match cli.command.unwrap_or(Commands::List) {
        Commands::List => list(cli.format),
        Commands::Version => version(cli.format),
    }
}

fn list(format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Text => {
            println!("list");
            println!("version");
        }
        OutputFormat::Json => {
            println!(r#"["list","version"]"#);
        }
    }
    Ok(())
}

fn version(format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Text => println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        OutputFormat::Json => println!(
            r#"{{"name":"{}","version":"{}"}}"#,
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ),
    }
    Ok(())
}
