#![allow(dead_code)]
#![allow(non_snake_case)]

fn main() {
  env_logger::init();

  let mut m = lc3::Machine::new();
  m.init();

  while !m.halt {
    m.step();
  }
}
