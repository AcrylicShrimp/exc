#[cfg(test)]
mod tests;

use clap::{Arg, ArgMatches, Command};
use colored::Colorize;
use exc_resolve::{resolve_modules, SourceFileResolver};
use std::path::PathBuf;
use thiserror::Error;

fn cli() -> Command {
    Command::new("exc")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .subcommand(
            Command::new("build").about("Builds a project").arg(
                Arg::new("INPUT")
                    .help("The path to the root module to build")
                    .required(true)
                    .index(1)
                    .value_parser(clap::value_parser!(PathBuf)),
            ),
        )
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("build", matches)) => match build(matches).await {
            Ok(_) => {}
            Err(err) => {
                eprintln!(
                    "{} Failed to build project: {}",
                    "[FATAL]".bold().red(),
                    err
                );
                return;
            }
        },
        _ => unreachable!(),
    };
}

#[derive(Error, Debug)]
enum BuildError {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("path `{0}` does not exist or is not accessible")]
    PathNotReachable(PathBuf),
    #[error("path `{0}` is not a file")]
    PathIsNotFile(PathBuf),
    #[error("path `{0}` is the root path; it must have a parent directory")]
    PathIsRoot(PathBuf),
    #[error("path `{0}` has no file name")]
    PathHasNoFileName(PathBuf),
    #[error("{0}")]
    SourceFileResolveError(#[from] exc_resolve::SourceFileResolveError),
}

async fn build(arg: &ArgMatches) -> Result<(), BuildError> {
    let input = arg.get_one::<PathBuf>("INPUT").unwrap();
    let absolute_path = std::env::current_dir()?.join(input);

    if !absolute_path.exists() {
        return Err(BuildError::PathNotReachable(absolute_path));
    }

    let absolute_path = absolute_path.canonicalize()?;

    if !std::fs::metadata(&absolute_path)?.is_file() {
        return Err(BuildError::PathIsNotFile(absolute_path));
    }

    let root_path = absolute_path
        .parent()
        .ok_or_else(|| BuildError::PathIsRoot(absolute_path.clone()))?;
    let file_name = absolute_path
        .file_name()
        .ok_or_else(|| BuildError::PathHasNoFileName(absolute_path.clone()))?;

    let mut source_file_resolver = SourceFileResolver::new(root_path);
    let root_module = source_file_resolver.resolve_file(file_name).await?;

    resolve_modules(&mut source_file_resolver, root_module).await;

    Ok(())
}
