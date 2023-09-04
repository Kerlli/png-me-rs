mod cli;

use cli::parse;
use cli::commands::Commands;

use png::Png;
use utils::fs::read_file_buffer;

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
}
