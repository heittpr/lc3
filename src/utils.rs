pub fn sign_extend(n: u16, size: usize) -> u16 {
  if (n>>(size-1))&1 == 0 {
    return n;
  }
  return n | (0xFFFF << size);
}
