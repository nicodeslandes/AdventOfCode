const INPUT_LEN: usize = 17;
pub type ResultSet = [u8; INPUT_LEN];

pub fn run(input: &str, results: &mut ResultSet, part: u8) {
    let mut input_parity: usize = 0;
    let mut parity: usize = 0;

    // Process the input
    // find input parity forward
    for (i, ch) in input.chars().enumerate() {
        parity ^= if ch == '1' { 1 } else { 0 };
        input_parity ^= parity << (i + 1);
    }

    // ...and reversed complement
    for (i, ch) in input.chars().rev().enumerate() {
        parity ^= if ch == '1' { 0 } else { 1 };
        input_parity ^= parity << (INPUT_LEN + i + 1);
    }

    match part {
        1 => solve(input_parity, 272, results),
        2 => solve(input_parity, 35651584, results),
        _ => panic!("Invalid part"),
    };
}

fn solve(input_parity: usize, disk_size: usize, results: &mut ResultSet) {
    let increment = find_lowest_1(disk_size);
    let mut previous_parity = 0;
    for length in (increment..=disk_size).step_by(increment) {
        // number of dragon bits
        let dragons = length / (INPUT_LEN + 1);
        // number of complete cycles (forward and reverse) of the input
        let input_cycles = (length - dragons) / (INPUT_LEN * 2);
        // remainder of input bits
        let input_remainder = (length - dragons) % (INPUT_LEN * 2);
        // parity of the dragon bits
        let mut p = dragon_parity(dragons);
        // plus parity of all complete input cycles
        p ^= input_cycles & INPUT_LEN;
        // plus parity of the remainder
        p ^= input_parity >> input_remainder;
        // only need the least significant bit
        p &= 1;
        // checksum digit is the inverted parity bit,
        // XOR with the previous parity calculation
        results[length / increment - 1] = (p ^ (if previous_parity == 0 { 1 } else { 0 })) as u8;
        previous_parity = p;
    }
}

// Returns lowest 1-bit in number
fn find_lowest_1(n: usize) -> usize {
    return n & (-(n as isize) as usize);
}

// Returns parity of dragon curve of length n
fn dragon_parity(n: usize) -> usize {
    let gray = n ^ (n >> 1);
    return (gray ^ ((n & gray).count_ones() as usize)) & 1;
}
