pub fn sign_extend(n: u16, size: usize) -> u16 {
  if (n >> (size-1)) & 0x1 == 0 {
      n
  } else {
      n | (0xFFFF << size)
  }
}
