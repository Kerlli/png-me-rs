mod cli;

use std::str::FromStr;
use cli::parse;
use cli::commands::Commands;

use png::Png;
use png::chunk::{Chunk, ChunkType};
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
    Commands::Set(args) => {
      let filepath = args.file;
      let chunk_name = args.chunk_name;
      let message = args.message;

      let buffer = read_file_buffer(&filepath);

      let mut png = Png::try_from(buffer.as_slice())
        .expect("Not a valid png format");

      let chunk = png.mut_chunk_by_type(&chunk_name);
      
      match chunk {
        Some(c) => {
          c.set_data(message.as_bytes());
        },
        None => {
          let chunk_type = ChunkType::from_str(&chunk_name)
            .expect("Invalid chunk type");
          let data = message.into_bytes();
          let chunk = Chunk::new(chunk_type, data);
          // find IEND
          let end_pos = png.chunk_position("IEND");

          match end_pos {
            Some(pos) => {
              png.insert_chunk(pos, chunk);
            },
            None => {
              eprintln!("Failed: can not found IEND chunk");
              return;
            }
          }
        },
      };

      write_buffer_to_file(&png.as_bytes()[..], &filepath);
      println!("Success");
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
    },
  };
}
