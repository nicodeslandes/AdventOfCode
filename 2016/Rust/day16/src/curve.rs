use crate::bits::Bits;
use std::fmt::Display;
use std::fmt::Formatter;

/// Curve is made of a repeated base pattern, followed by some remaining bits
/// C = (Base)^m . tail
pub struct Curve {
    //base: Bits,
    multiplier: usize,
    tail: Bits,
    len: usize,
}

impl Display for Curve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        if self.multiplier == 0 {
            write!(f, "{} (len: {})", self.tail, self.len)
        } else {
            write!(
                f,
                "({}^{}).{} (len: {})",
                self.tail | Bits::single(0),
                self.multiplier,
                self.tail,
                self.len
            )
        }
    }
}
impl Curve {
    pub fn new(bits: &str) -> Curve {
        let x = Bits::parse(bits);
        Curve {
            tail: x,
            multiplier: 0,
            len: x.len, //base: Bits::single(0),
        }
    }
    pub fn expand_and_trim(&mut self, len: usize) {
        // We start off with just a tail
        if self.multiplier > 0 {
            panic!("Unexpected multiplier: {0}", self.multiplier)
        }

        // The expanded curve will be A^n . B
        // where A = T.0.rev(T).0
        //   and B = T.0.rev(T)
        //   and n = 2^k - 1, k > 1

        // And the resulting size is L = 2^(k+1).(x + 1) - 1, with x = (len(T)-1)/2
        // ie: 2^(k+1) = (L+1)/(x + 1)
        // <=> k = log2((L + 1)/(x + 1)) - 1
        let x = self.tail.len;
        let log = ((len + 1) as f64 / ((x + 1) as f64)).log2() - 1.0;
        println!("Log: {}", log);
        let k = log.ceil() as usize;
        let t = self.tail;
        let b = t | Bits::single(0) | t.rev_inv();
        println!("Old: {}, New bits: {}", t, b);
        //let a = b | Bits::single(0);

        //self.base = a;
        self.tail = b;
        self.multiplier = (1 << k) - 1;
        self.len = len;
    }
}

// Idea: Have this iterator generate the mask directly instead of the individual bits
pub struct SingleBitCurve {
    buffer: Vec<u8>,
    mirror_ptr: isize,
    current_ptr: isize,
}

impl SingleBitCurve {
    pub fn new(size: usize) -> SingleBitCurve {
        SingleBitCurve {
            buffer: vec![0; size],
            mirror_ptr: -1,
            current_ptr: -1,
        }
    }
}

impl Iterator for SingleBitCurve {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        if self.current_ptr + 1 >= self.buffer.len().try_into().unwrap() {
            return None;
        }
        if self.mirror_ptr < 0 {
            // We've reached the end of the items to mirror; the new value is the added 0 between patterns
            self.mirror_ptr = self.current_ptr;
            self.current_ptr += 1;
            return Some(0);
        }

        // Mirror the element pointed to by self.mirror_ptr
        self.current_ptr += 1;
        let value = 1 - self.buffer[self.mirror_ptr as usize];
        self.buffer[self.current_ptr as usize] = value;
        self.mirror_ptr -= 1;
        Some(value)
    }
}
