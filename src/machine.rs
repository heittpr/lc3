use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use utils::sign_extend;

#[derive(FromPrimitive)]
#[repr(u16)]
enum OP {
  BR,   // branch
  ADD,  // add
  LD,   // load
  ST,   // store
  JSR,  // jump register
  AND,  // bitwise and
  LDR,  // load register
  STR,  // store register
  RTI,  // unused
  NOT,  // bitwise not
  LDI,  // load indirect
  STI,  // store indirect
  JMP,  // jump
  RES,  // unused
  LEA,  // load effective address
  TRAP, // execute trap
}

pub const MEM_SIZE: usize = 1<<16;
pub const REG_SIZE: usize = 10;

pub const PC    : u16 = 8;
pub const COND  : u16 = 9;
pub const POS   : u16 = 1 << 0;
pub const ZRO   : u16 = 1 << 1;
pub const NEG   : u16 = 1 << 2;

pub struct Machine {
  pub reg: [u16; REG_SIZE],
  pub mem: [u16; MEM_SIZE],
  pub halt: bool,
}

impl Machine {
  pub fn new() -> Machine {
    Machine {
      reg: [0; REG_SIZE],
      mem: [0; MEM_SIZE],
      halt: true,
    }
  }
  
  pub fn init(&mut self) {
    self.halt = false;
    self.setr(PC, 0x3000);
  }
  
  fn getr(&self, r: u16) -> u16 {
    self.reg[r as usize]
  }

  fn setr(&mut self, r: u16, val: u16) {
    self.reg[r as usize] = val;
  }

  fn addr(&mut self, r: u16, val: u16) {
    self.reg[r as usize] += val;
  }

  fn getm(&self, addr: u16) -> u16 {
    self.mem[addr as usize]
  }
  
  fn setm(&mut self, addr: u16, val: u16){
    self.mem[addr as usize] = val;
  }

  fn update_cond(&mut self, r: u16) {
    let val: u16 = self.getr(r);

    if val == 0 {
      self.setr(COND, ZRO);
    } else if (val as i16) > 0 {
      self.setr(COND, POS);
    } else {
      self.setr(COND, NEG);
    }
  }

  pub fn step(&mut self) {
    let instr: u16 = self.getm(self.getr(PC));

    if let Some(op) = OP::from_u16(instr >> 12) {
      match op {
        OP::ADD => {
          let dr: u16 = (instr >> 9) & 0x7;
          let sr1: u16 = (instr >> 6) & 0x7;

          if (instr >> 5) & 0x1 == 1 {
            let imm: u16 = sign_extend(instr & 0x1F, 5);
            self.setr(dr, self.getr(sr1) + imm);
          } else {
            let sr2: u16 = instr & 0x7;
            self.setr(dr, self.getr(sr1) + self.getr(sr2));
          }

          self.update_cond(dr);
        },

        OP::LDI => {
          let dr: u16 = (instr >> 9) & 0x7;
          let offset: u16 = sign_extend(instr & 0x1FF, 9);

          self.setr(dr, self.getm(self.getm(self.getr(PC) + offset)));
        },

        OP::BR => {
          let n: bool = ((instr >> 11) & 0x1) == 0;
          let z: bool = ((instr >> 10) & 0x1) == 0;
          let p: bool = ((instr >>  9) & 0x1) == 0;
          let offset: u16 = sign_extend(instr & 0x1FF, 9);

          let N: bool = self.getr(COND) == NEG;
          let Z: bool = self.getr(COND) == ZRO;
          let P: bool = self.getr(COND) == POS;

          if (n && N) || (z && Z) || (p || P) {
            self.addr(PC, offset);
          }
        },

        OP::JMP => {
          let base: u16 = (instr >> 6) & 0x7;
          self.setr(PC, base);
        },

        OP::JSR => {
          self.setr(0x7, self.getr(PC)); 

          if (instr >> 11) & 0x1 == 1 {
            let offset: u16 = sign_extend(instr & 0xFFF, 11);
            self.addr(PC, offset);
          } else {
            let base: u16 = (instr >> 6) & 0x7;
            self.setr(PC, base);
          }
        },

        _ => panic!("not implemented {:#x}", instr),
      }
    } else {
      panic!("invalid opcode! {:#x}", instr);
    }
  }
}
