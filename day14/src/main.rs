use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let parsed_input: Vec<(Bitmask, WriteOps)> = parse_input(input);

    // part 1
    let mut memory: HashMap<usize, u64> = HashMap::new();
    for (bitmask, program) in parsed_input.clone() {
        for (address, write_value) in program {
            let m = memory.entry(address).or_insert(0);
            *m = apply_mask(write_value, &bitmask);
        }
    }
    println!("part 1 sum of memory: {}", memory.values().sum::<u64>());

    // part 2
    let mut memory: HashMap<usize, u64> = HashMap::new();
    for (bitmask, program) in parsed_input {
        for (address, write_value) in program {
            let addresses = mask_to_addresses(address, &bitmask);
            for write_to_address in addresses {
                let m = memory.entry(write_to_address).or_insert(0);
                *m = write_value;
            }
        }
    }
    println!("part 2 sum of memory: {}", memory.values().sum::<u64>());
}

type Bitmask = String;
type WriteOps = Vec<(usize, u64)>;

fn parse_input(input: &str) -> Vec<(Bitmask, WriteOps)> {
    let mut output: Vec<(Bitmask, WriteOps)> = Vec::new();
    let mut current_mask: String = "".into();
    let mut current_actions = Vec::new();

    for line in input.lines() {
        if line.starts_with("mask = ") {
            if !current_mask.is_empty() {
                output.push((current_mask.clone(), current_actions));
                current_actions = Vec::new();
            }
            let re = Regex::new(r"^mask = ([10X]{36})$").unwrap();
            let caps = re.captures(line).unwrap();
            current_mask = caps.get(1).unwrap().as_str().to_string();
        } else {
            let re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
            let caps = re.captures(line).unwrap();
            let address = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let value = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
            current_actions.push((address, value));
        }
    }
    output.push((current_mask, current_actions));
    output
}

fn apply_mask(mut val: u64, mask: &str) -> u64 {
    let mask_ops: Vec<(usize, bool)> = mask
        .chars()
        .rev()
        .enumerate()
        .filter_map(|(idx, bitval)| {
            match bitval {
                '1' => Some((idx, true)),
                '0' => Some((idx, false)),
                _ => None
            }
        }).collect();

    for mask_op in mask_ops {
        match mask_op {
            (bit, true) => val = turn_bit_on(val, bit),
            (bit, false) => val = turn_bit_off(val, bit)
        }
    }

    val
}

fn turn_bit_on(val: u64, idx_of_bit: usize) -> u64 {
    let mask = 1 << idx_of_bit;
    val | mask
}

fn turn_bit_off(val: u64, idx_of_bit: usize) -> u64 {
    let mask = 1 << idx_of_bit;
    val & !mask
}

// given address of  000000000000000000000000000000101010  (decimal 42)
// and a mask of     000000000000000000000000000000X1001X
// results in a mask of
//                   000000000000000000000000000000X1101X
// which returns these addresses:
// 000000000000000000000000000000011010  (decimal 26)
// 000000000000000000000000000000011011  (decimal 27)
// 000000000000000000000000000000111010  (decimal 58)
// 000000000000000000000000000000111011  (decimal 59)
//
// this is probably very convoluted way of doing this in rust, but it already took
// too much time for today, so not refactoring this any further for now.
//
fn mask_to_addresses(address: usize, mask: &str) -> Vec<usize> {
    let mut output = Vec::new();
    let bin_val_str = format!("{:036b}", address);
    let result_mask = (bin_val_str.chars().zip(mask.chars())).map(|(v, m)| {
        match (v, m) {
            ('0', '0') => '0',
            ('0', '1') => '1',
            ('1', '0') => '1',
            ('1', '1') => '1',
            (_, 'X') => 'X',
            unhandled => panic!("unhandled! {:?}", unhandled),
        }
    }).collect::<Vec<char>>();

    // find indices for all X chars in result_mask
    let mut idxs = Vec::new();
    let mut x_iter = result_mask.iter();
    while let Some(idx) = x_iter.rposition(|x| x == &'X') {
        idxs.push(idx);
    }

    let mut bss = BinaryStringStream::new(idxs.len());
    while let Some(bits) = bss.next() {
        let mut result_mask2 = result_mask.clone();
        // bits = ['0', '1']
        for (bits_idx, mask_idx) in idxs.iter().enumerate() {
            result_mask2[*mask_idx] = bits[bits.len() - 1 - bits_idx];
        }
        let s: String = result_mask2.into_iter().collect();
        output.push(usize::from_str_radix(&s, 2).unwrap());
    }

    output
}

// trying out something new; custom iterator that spits out a stream of binary strings
// to be used in the mask_to_addresses function
struct BinaryStringStream {
    bit_count: usize,
    curr: u32,
}

impl BinaryStringStream {
    fn new(bit_count: usize) -> Self {
        Self {
            bit_count,
            curr: 0,
        }
    }
}

impl Iterator for BinaryStringStream {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let bin_val_str = format!("{:b}", self.curr);
        self.curr += 1;

        let mut vec_chars = bin_val_str.chars().collect::<Vec<char>>();
        if vec_chars.len() > self.bit_count {
            None
        } else {
            for _i in 0..(self.bit_count - vec_chars.len()) {
                vec_chars.insert(0, '0');
            }
            Some(vec_chars)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_to_addresses() {
        let outcome = mask_to_addresses(42, "000000000000000000000000000000X1001X");
        let expected = vec![
            0b000000000000000000000000000000011010,
            0b000000000000000000000000000000011011,
            0b000000000000000000000000000000111010,
            0b000000000000000000000000000000111011,
        ];

        assert_eq!(outcome, expected)
    }

    #[test]
    fn test_mask_to_addresses2() {
        let outcome = mask_to_addresses(26, "00000000000000000000000000000000X0XX");
        let expected = vec![
            0b000000000000000000000000000000010000,
            0b000000000000000000000000000000010001,
            0b000000000000000000000000000000010010,
            0b000000000000000000000000000000010011,
            0b000000000000000000000000000000011000,
            0b000000000000000000000000000000011001,
            0b000000000000000000000000000000011010,
            0b000000000000000000000000000000011011,
        ];

        assert_eq!(outcome, expected)
    }
}