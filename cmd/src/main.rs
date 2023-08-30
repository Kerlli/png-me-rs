mod cli;

use cli::parse;
use cli::commands::Commands;

use png::Png;
// use png::ChunkType;
// use png::chunk::Chunk;
use utils::fs::read_file_buffer;
// use std::str::FromStr;

fn main() {
  let command = parse();

  match command {
    Commands::Info(info) => {
      let filepath = info.input;

      let buffer = read_file_buffer(&filepath);

      let png = Png::try_from(buffer.as_slice())
        .expect("Not a valid png format");

      println!("{}", png.to_string());
    }
  };

  // remove IEND
  // let iend_chunk: Chunk = sample_png_meta.remove_chunk("IEND").unwrap();

  // let chunk_type = ChunkType::from_str(&tag).unwrap();

  // let new_chunk = Chunk::new(chunk_type, message.into_bytes());

  // sample_png_meta.append_chunk(new_chunk);
  // sample_png_meta.append_chunk(iend_chunk);

  // let output_png_bytes = sample_png_meta.as_bytes();

  // let mut output_file = File::create(filepath).expect("Create file error");
  // output_file.write_all(&output_png_bytes).expect("Write file failed");

  // println!("Process succeed");
}
