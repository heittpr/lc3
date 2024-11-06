#![allow(dead_code)]
#![allow(non_snake_case)]

fn main() {
  let mut m = lc3::Machine::new();

  m.init();

  while !m.halt {
    m.step();
    m.halt = true;
  }
}
