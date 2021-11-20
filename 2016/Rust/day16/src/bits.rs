use gen_iter::GenIter;
use std::fmt;
use std::fmt::Display;
use std::fmt::Write;
use std::ops::BitOr;

#[derive(Copy, Clone)]
pub struct Bits {
    pub bits: u64,
    pub len: usize,
}

macro_rules! gen_iter_move {
    ($block: block) => {
        GenIter(move || $block)
    };
}

impl Bits {
    pub fn parse(s: &str) -> Bits {
        println!("Parsing {}", s);

        let bits = s.chars().fold(0, |acc, ch| {
            (acc << 1)
                + (match ch {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!("Unexpected character: {}", ch),
                })
        });
        Bits {
            bits: bits,
            len: s.len(),
        }
    }
    pub fn single(value: u8) -> Bits {
        if value > 1 {
            panic!("Invalid bit value: {0}", value)
        }
        Bits {
            bits: value as u64,
            len: 1,
        }
    }
    pub fn rev_inv(&self) -> Bits {
        // eg: bits: 11010
        //     len: 5

        let mut rev_bits = 0;
        let mut bits = self.bits;
        for _ in 0..self.len {
            rev_bits <<= 1;
            if bits % 2 == 0 {
                rev_bits |= 1;
            }
            bits >>= 1;
        }

        Bits {
            bits: rev_bits,
            len: self.len,
        }
    }

    pub fn to_string(&self) -> String {
        let output = self
            ._get_bits()
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| match chunk {
                [0, 0] => 'a',
                [0, 1] => 'b',
                [1, 0] => 'c',
                [1, 1] => 'd',
                [0] => 'Y',
                [1] => 'Z',
                x => panic!("Unexpected value: {:?}", x),
            })
            .fold(String::new(), |s, ch| format!("{}{}", s, ch));

        output
    }

    fn _get_bits(&self) -> impl Iterator<Item = i32> {
        let bits = self.bits;
        let len = self.len;

        gen_iter_move!({
            let mut mask = 1 << (len - 1);
            while mask > 0 {
                yield if bits & mask == 0 { 0 } else { 1 };
                mask >>= 1;
            }
        })
    }
}

impl BitOr for Bits {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        let len = self.len + other.len;
        if len > 64 {
            panic!(
                "Cannot join such big Bits! Joining {} and {}; resulting len: {}",
                self, other, len
            )
        }
        let mut bits = self.bits;
        bits <<= other.len;
        bits |= other.bits;
        Self {
            bits: bits,
            len: len,
        }
    }
}

impl Display for Bits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        let mut mask = 1 << self.len;

        while mask > 1 {
            mask >>= 1;
            f.write_char(if self.bits & mask != 0 { '1' } else { '0' })?
        }

        // f.write_str(" (")?;
        // f.write_str(&format!("{}", self.len))?;
        // f.write_str(&self.to_string())?;
        // f.write_char(')')?;
        Ok(())
    }
}
