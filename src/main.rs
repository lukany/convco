#[macro_use]
extern crate lazy_static;

mod cli;
mod cmd;
mod conventional;
mod git;

use std::io;

use crate::cmd::Command;

use handlebars::{RenderError, TemplateError};
use std::process::exit;
use structopt::StructOpt;

#[derive(Debug)]
enum Error {
    Git(git2::Error),
    Io(io::Error),
    Template(TemplateError),
    Render(RenderError),
    Check,
}

impl From<git2::Error> for Error {
    fn from(e: git2::Error) -> Self {
        Self::Git(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<TemplateError> for Error {
    fn from(e: TemplateError) -> Self {
        Self::Template(e)
    }
}

impl From<RenderError> for Error {
    fn from(e: RenderError) -> Self {
        Self::Render(e)
    }
}

fn main() -> Result<(), Error> {
    let opt: cli::Opt = cli::Opt::from_args();
    if let Some(path) = opt.path {
        std::env::set_current_dir(path)?;
    }
    let res = match opt.cmd {
        cli::Command::Check(cmd) => cmd.exec(),
        cli::Command::Changelog(cmd) => cmd.exec(),
        cli::Command::Version(cmd) => cmd.exec(),
    };
    match res {
        Err(e) => {
            match e {
                Error::Check => (),
                _ => eprintln!("{:?}", e),
            }
            exit(1)
        }
        _ => exit(0),
    }
}