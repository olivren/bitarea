#![feature(core)]

pub mod bitarea {

use std::ops::Shl;
use std::u64;
use std::fmt;

#[derive(Copy)]
pub struct Bitarea64 {
    pub data: u64,
    pub width: u32,
    pub height: u32
}

impl Bitarea64 {

    pub fn new(width: u32, height: u32) -> Bitarea64 {
        assert!(u64::BITS >= width*height);
        Bitarea64 {
            data: 0,
            width: width,
            height: height
        }
    }

    pub fn set(&mut self, col: u32, row:u32, val: bool) {
        assert!(row < self.height && col < self.width);
        let mask = 1 << (63-(row*self.width+col));
        self.data = if val {
            self.data | mask
        } else {
            self.data & !mask
        }
    }

    pub fn get(self, col: u32, row:u32) -> bool {
        assert!(row < self.height && col < self.width);
        let mask = 1 << (63-(row*self.width+col));
        self.data & mask != 0
    }
}

impl Shl<u32> for Bitarea64 {

    type Output = Bitarea64;

    fn shl(self, rhl: u32) -> Bitarea64 {
        let mut mask = !0u64;
        for i in 0..self.height {
            mask &= !(1 << (64-i*self.width));
        }
        Bitarea64 {
            data: (self.data << rhl) & mask,
            ..self
        }
    }
}

impl fmt::Debug for Bitarea64 {

    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for r in 0..self.height {
            for c in 0..self.width {
                try!(f.write_str(if self.get(c,r) {"1"} else {"0"}));
            }
            try!(f.write_str("\n"));
        }
        Ok(())
    }
}

#[test]
fn test1() {
    let mut b1 = Bitarea64::new(3,4);
    b1.set(0,0, true);
    b1.set(1,3, true);
    b1.set(1,2, true);
    b1.set(0,2, true);
    b1.set(2,1, true);
    println!("{:?}", b1);

    let b2 = b1 << 1;
    println!("{:?}", b2);

}

}
