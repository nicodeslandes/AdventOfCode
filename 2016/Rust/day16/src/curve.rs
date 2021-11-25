use crate::bits::Bits;
use std::fmt::Display;
use std::fmt::Formatter;
use std::option::Option;

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

    pub fn compress(&self) -> Vec<u8> {
        // Work out the target size
        let mut len = self.len;
        let mut buf_size = 1;
        while len % 2 == 0 {
            len >>= 1;
            buf_size <<= 1;
        }

        println!("Allocation buffers: buffer: {}, result: {}", buf_size, len);
        let mut result = vec![0; len];
        let buffer = vec![0; buf_size];

        let corrections = SingleBitCurve::new(buf_size);
        for r in result.iter_mut() {}

        result
    }
}

struct CurveDataIterator<'a> {
    curve: &'a Curve,
    pos: usize,
    buffer: Vec<u8>,
}

impl<'a> CurveDataIterator<'a> {
    fn new(curve: &Curve) -> CurveDataIterator {
        CurveDataIterator {
            curve: curve,
            pos: 0,
            buffer: vec![0; 2 * 1024 * 1024 / 8],
        }
    }
}

impl<'a> Iterator for CurveDataIterator<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.curve.len {
            return None;
        }

        // curve is (t0)^m . t
        let pattern_len = self.curve.tail.len + 1;

        let offset = self.pos % self.curve.multiplier;

        // first chunk to put in the buffer is the tail part after the offset
        //
        // <--------- 64 bits -------->
        // 0000000000000011111010011101
        //              |        |    |
        //              <---- l ------>
        //              |-- o -->|
        //                 chunk:|<-->|

        let chunk_size = 64 - (self.curve.tail.len - offset);
        let mut chunk = self.curve.tail.bits << 64 - chunk_size;

        // That gives us the chunk all shifted to the left
        // |111010000000000|
        // OR with rest of tail, with 0 prepended
        // |000000111110100|
        // |111010111110100|
        // This we replicate across the 2MB buffer
        // |111010111110100111010111110100111010111110100111010111110100111010111110100111010111110100|...
        // with correcting bits:
        // |      1      0      1      1      0      1      0      0      0      1      1      1      |

        // tail:       01110110101001000 0 11101101010010001 0 01110110101001000 0 1110110101 0010001 0 01110110101001000 0 11101101010010001 0 ...
        //                     x         0        x^         0
        // Correction:                   0                   0                   1           |        0                   1
        // aligned     01110110101001000 0 11101101010010001 0 01110110101001000 1 1110110101|0010001 0 01110110101001000 0 11101101010010001

        //            36|36|36|
        // Aligned:   36-28|8-36-20|16-36-12|24

        Some(vec![1, 2, 3])
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
