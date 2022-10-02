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
    #[arg(required(true))]
    pub command: Vec<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}
