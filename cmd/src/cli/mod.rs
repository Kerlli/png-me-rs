pub mod commands;

use commands::Commands;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  command: Commands,
}

pub fn parse() -> Commands {
  // ruSt
  Cli::parse().command

  // let tag = &args.tag;

  // if tag.as_bytes().len() !=4 {
  //   let mut cmd = Args::command();
  //   return Err(cmd.error(ErrorKind::InvalidValue, "Invalid Tag"))
  // }

  // Ok(cli)
}
