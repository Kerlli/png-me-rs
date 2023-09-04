mod cli;

use cli::parse;
use cli::commands::Commands;

use png::Png;
use utils::fs::{read_file_buffer, write_buffer_to_file};

fn main() {
  let command = parse();

  match command {
    Commands::Info(args) => {
      let filepath = args.file;

      let buffer = read_file_buffer(&filepath);

      let png = Png::try_from(buffer.as_slice())
        .expect("Not a valid png format");
      
      println!("{}", png.to_string());
    },
    Commands::Remove(args) => {
      let filepath = args.file;
      let chunk_name = args.chunk_name;

      let buffer = read_file_buffer(&filepath);

      let mut png = Png::try_from(buffer.as_slice())
        .expect("Not a valid png format");

      match png.remove_chunk(&chunk_name) {
        Ok(_) => {
          write_buffer_to_file(&png.as_bytes()[..], &filepath);
          println!("Successfully removed chunk {}", chunk_name);
        },
        Err(e) => {
          println!("Failed to remove chunk {}: {}", chunk_name, e);
        },
      };
    }
  };
}
