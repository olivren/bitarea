extern crate rand;

pub mod bitarea {

use std::ops::{Shl,Shr};
use std::fmt;

const width: u32 = 3;
const height: u32 = 4;
const used_bits: u32 = width*height;
const unused_bits: u32 = 64 - used_bits;

#[derive(Copy)]
pub struct Bitarea {
    pub data: u64,
}

impl Bitarea {

    pub fn new() -> Bitarea {
        Bitarea {data: 0}
    }

    pub fn set(&mut self, col: u32, row:u32, val: bool) {
        assert!(row < height && col < width);
        let mask = 1 << (63-(row*width+col));
        self.data = if val {
            self.data | mask
        } else {
            self.data & !mask
        }
    }

    pub fn get(self, col: u32, row:u32) -> bool {
        assert!(row < height && col < width);
        let mask = 1 << (63-(row*width+col));
        self.data & mask != 0
    }

    pub fn from_parts(parts: &[u64]) -> Bitarea {
        assert!(parts.len() == height as usize);
        let mut d = 0;
        for v in parts.iter() {
            d <<= width;
            d |= *v;
        }
        d <<= unused_bits;
        Bitarea {data: d}
    }
}

impl Shl<u32> for Bitarea {

    type Output = Bitarea;

    fn shl(self, rhl: u32) -> Bitarea {
        let mut mask = 0;
        for i in 0..height {
            mask <<= width;
            mask |= 1;
        }
        mask = (!mask) << unused_bits;
        Bitarea {data: (self.data << rhl) & mask}
    }
}

impl Shr<u32> for Bitarea {

    type Output = Bitarea;

    fn shr(self, rhl: u32) -> Bitarea {
        let mut mask = !0u64;
        for i in 0..height {
            mask &= !((1 << width-1) << (64-i*width));
        }
        Bitarea {data: (self.data >> rhl) & mask}
    }
}

impl fmt::Debug for Bitarea {

    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for r in 0..height {
            try!(f.write_str("\n"));
            for c in 0..width {
                try!(f.write_str(if self.get(c,r) {"1"} else {"0"}));
            }
        }
        Ok(())
    }
}

impl Eq for Bitarea{}
impl PartialEq for Bitarea {

    fn eq(&self, other: &Bitarea) -> bool {
        return (self.data >> unused_bits) == (other.data >> unused_bits);
    }
}

#[test]
fn test_fmt() {

    let x = Bitarea::from_parts(&[0b100,
                                  0b001,
                                  0b110,
                                  0b010]);
    assert_eq!("\n100\n001\n110\n010", format!("{:?}", x));
}

#[test]
fn test_set() {
    let mut b = Bitarea::new();
    b.set(0,0, true);
    b.set(1,3, true);
    b.set(1,2, true);
    b.set(0,2, true);
    b.set(2,1, true);

    assert_eq!(Bitarea::from_parts(&[0b100,
                                     0b001,
                                     0b110,
                                     0b010]),
               b)
}

#[test]
fn test_get() {
    let b = Bitarea::from_parts(&[0b001,
                                  0b111,
                                  0b010,
                                  0b001]);
    assert_eq!(b.get(0,0), false);
    assert_eq!(b.get(0,1), true);
    assert_eq!(b.get(0,2), false);
    assert_eq!(b.get(0,3), false);
    assert_eq!(b.get(1,0), false);
    assert_eq!(b.get(1,1), true);
    assert_eq!(b.get(1,2), true);
    assert_eq!(b.get(1,3), false);
    assert_eq!(b.get(2,0), true);
    assert_eq!(b.get(2,1), true);
    assert_eq!(b.get(2,2), false);
    assert_eq!(b.get(2,3), true);
}

#[test]
fn test_shl() {
    let b = Bitarea::from_parts(&[0b001,
                                  0b111,
                                  0b010,
                                  0b001]);

    assert_eq!(Bitarea::from_parts(&[0b010,
                                     0b110,
                                     0b100,
                                     0b010]),
               b << 1);

    assert_eq!(Bitarea::from_parts(&[0b100,
                                     0b100,
                                     0b000,
                                     0b100]),
               b << 2);

    assert_eq!(Bitarea::from_parts(&[0b000,
                                     0b000,
                                     0b000,
                                     0b000]),
               b << 3);

    assert_eq!(Bitarea::from_parts(&[0b000,
                                     0b000,
                                     0b000,
                                     0b000]),
               b << 4);

}

#[test]
fn test_shr() {
}

}
