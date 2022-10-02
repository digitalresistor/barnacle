use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// Jinja2 template to render
    #[arg()]
    pub template: PathBuf,

    /// The location to place the rendered file
    #[arg()]
    pub output: PathBuf,

    /// The command + arguments to execute once template is rendered
    #[arg(group = "command_or_exit", required(true))]
    pub command: Vec<String>,

    /// Exit with error code, exclusive with a command to exeute
    #[arg(short, long, group = "command_or_exit", required(true))]
    pub exit: Option<u8>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}
