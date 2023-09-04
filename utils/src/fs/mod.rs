use std::fs::File;
use std::io::{BufReader, Read, Write};

pub fn read_file_buffer(filepath: &str) -> Vec<u8> {
  let file = File::open(filepath).expect("Open file error");

  let buffer_size = 1024;

  let mut reader = BufReader::with_capacity(buffer_size, file);
  let mut buffer: Vec<u8> = vec![];

  reader.read_to_end(&mut buffer).expect("Read buffer error");

  buffer
}

pub fn write_buffer_to_file(buf: &[u8], filepath: &str) {
  let mut file = File::create(filepath).expect("Create file error");

  file.write_all(buf).expect("Write buf to file error");
}
