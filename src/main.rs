use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;
use std::{fs, os::unix::process::CommandExt};

use anyhow::Result;
use clap::Parser;
use minijinja::{context, Environment, Source};

use barnacle::cli::Cli;
use barnacle::error::Error;

fn exec(command: &[String]) {
    let prog = &command[0];
    let args = &command[1..];

    Command::new(prog).args(args).exec();
}

fn jinja_environment(template_path: &Path) -> Result<Environment> {
    let mut env = Environment::new();
    let mut source = Source::new();
    let input = fs::read_to_string(template_path).map_err(Error::FileMissing)?;

    source.add_template("config.j2", input)?;
    env.set_source(source);

    Ok(env)
}

fn main() {
    let matches = Cli::parse();

    let env = jinja_environment(&matches.template).unwrap();
    let env_vars = std::env::vars().collect::<BTreeMap<_, _>>();
    let tmpl = env
        .get_template("config.j2")
        .expect("Failed to get config jinja2 template");
    let output = tmpl.render(context!(env => env_vars)).unwrap();

    fs::write(&matches.output, output).expect("Failed to write output file");

    // If we have an exit code, use it
    if let Some(exit) = matches.exit {
        std::process::exit(exit.into());
    }

    // Otherwise we execute the command provided on the CLI
    exec(&matches.command);
}
