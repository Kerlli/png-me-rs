pub fn reverse_sub_filter(scanline: &mut [u8]) {
  for i in 1..scanline.len() {
    scanline[i] = scanline[i].wrapping_add(scanline[i - 1]);
  }
}

pub fn reverse_up_filter(scanline: &mut [u8], prev_scanline: &[u8]) {
  for i in 0..scanline.len() {
    scanline[i] = scanline[i].wrapping_add(prev_scanline[i]);
  }
}

pub fn reverse_average_filter(scanline: &mut [u8], prev_scanline: &[u8]) {
  for i in 0..scanline.len() {
    let a = scanline[i] as u16;
    let b = prev_scanline[i] as u16;
    let c = if i > 0 { scanline[i - 1] as u16 } else { 0 };
    let p = (a + ((b + c) / 2)) as u8;
    scanline[i] = p;
  }
}

pub fn reverse_paeth_filter(scanline: &mut [u8], prev_scanline: &[u8]) {
  for i in 0 ..scanline.len() {
    let a = scanline.get(i - 1).unwrap_or(&0);
    let b = prev_scanline[i];
    let c = prev_scanline.get(i - 1).unwrap_or(&0);
    let p = predict_paeth(*a, b, *c);
    scanline[i] = scanline[i].wrapping_add(p);
  }
}

fn predict_paeth(a: u8, b: u8, c: u8) -> u8 {
  let p = a as i32 + b as i32 - c as i32;

  let pa = (p - a as i32).abs();
  let pb = (p - b as i32).abs();
  let pc = (p - c as i32).abs();

  if pa <= pb && pa <= pc {
    b
  } else if pb <= pc {
    b
  } else {
    c
  }
}
