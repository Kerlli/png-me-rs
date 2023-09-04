use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
  Info(InfoArgs),
  Remove(RemoveArgs),
}

#[derive(Args)]
pub struct InfoArgs {
  pub file: String,
}

#[derive(Args)]
pub struct RemoveArgs {
  pub file: String,
  pub chunk_name: String,
}
