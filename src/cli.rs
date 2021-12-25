use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    /// Jinja2 template to render
    #[clap(parse(from_os_str))]
    pub template: PathBuf,

    /// The location to place the rendered file
    #[clap(parse(from_os_str))]
    pub output: PathBuf,

    /// The command + arguments to execute once template is rendered
    #[clap(required(true))]
    pub command: Vec<String>,

    /// Turn debugging information on
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: usize,
}
