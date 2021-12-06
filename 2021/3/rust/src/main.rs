use helpers::Opt;

use std::io;

fn gamma_bit(bit_1_count: usize, half_of_total: usize) -> char {if bit_1_count >= half_of_total { '1' } else { '0' }}

fn epsilon_bit(bit_1_count: usize, half_of_total: usize) -> char {if bit_1_count >= half_of_total { '0' } else { '1' }}

fn get_rate(bits: &Vec<usize>, half: usize, filter: fn (usize, usize) -> char) -> usize {
    usize::from_str_radix(
        &bits.iter().map(|&b| filter(b, half)).collect::<String>(),
        2
    ).expect("Error converting from binary in string")
}

fn get_more_common_bits_value(numbers: &Vec<Vec<char>>) -> Vec<usize> {
    let mut bits = Vec::new();
    for line in numbers {
        while bits.len() < line.len() { bits.push(0) }
        for (index, &char) in line.iter().enumerate() {
            if char == '1' { bits[index] += 1 }
        }
    }
    return bits;
}

fn main() -> io::Result<()> {
    let numbers: Vec<Vec<char>> = Opt::from_args().lines()?.map(|line| line.chars().collect::<Vec<char>>()).collect();
    let bits = get_more_common_bits_value(&numbers);
    let half = numbers.len() / 2;
    let gamma_rate = get_rate(&bits, half, gamma_bit);
    let epsilon_rate = get_rate(&bits, half, epsilon_bit);
    println!("{}", gamma_rate * epsilon_rate);
    return Ok(());
}
