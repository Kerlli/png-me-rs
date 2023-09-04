use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
  Info(InfoArgs),
  Set(SetArgs),
  Remove(RemoveArgs),
}

#[derive(Args)]
pub struct InfoArgs {
  pub file: String,
}

#[derive(Args)]
pub struct SetArgs {
  pub file: String,
  pub chunk_name: String,
  pub message: String,
}

#[derive(Args)]
pub struct RemoveArgs {
  pub file: String,
  pub chunk_name: String,
}
