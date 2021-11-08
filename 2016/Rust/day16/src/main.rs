fn get_checksum_string(items: &Vec<char>, size: usize) -> String {
    items.iter().take(size).fold(String::new(), |s, &ch| {
        format!("{}{}", s, ch)
    })
}

fn checksum(items: &mut Vec<char>, size: usize) -> String {
    let mut size = size;
    loop {
        // Size is guaranteed to be even here
        println!("Checksum: {}", get_checksum_string(items, size));

        let mut index_in = 0;
        let mut index_out = 0;
        while index_in < size {
            items[index_out] = if items[index_in] == items[index_in + 1] {
                '1'
            } else {
                '0'
            };

            index_in += 2;
            index_out += 1;
        }
        size /= 2;
        if size % 2 == 1 {
            break;
        }
    }

    return get_checksum_string(items, size);
}

fn main() {
    let target_size = 272;

    

    let mut bits: Vec<char> = "01110110101001000".chars().collect();
    let mut n = bits.len();
    bits.extend(vec!['0';1000].iter());
 
    //println!("{} ({})", get_checksum_string(&bits, n), n);

    while n < target_size {
        bits[n] = '0';
        for i in 1..(n + 1) {
            bits[n + i] = if bits[n - i] == '0' { '1' } else { '0' };
        }

        n = n * 2 + 1;
        println!("{} ({})", get_checksum_string(&bits, n), n)
    }

    let result = checksum(&mut bits, target_size);
    println!("Result: {}", result);
}
