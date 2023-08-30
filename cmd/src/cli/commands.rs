use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
  Info(InfoArgs)
}

#[derive(Args)]
pub struct InfoArgs {
  pub input: String
}
