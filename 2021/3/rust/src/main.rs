use helpers::Opt;

use std::io;

type BitFilter = fn (usize, usize) -> char;

fn gamma_bit(bit_1_count: usize, total: usize) -> char {
    let bit_0_count = total - bit_1_count;
    if bit_1_count >= bit_0_count { '1' } else { '0' }
}

fn epsilon_bit(bit_1_count: usize, total: usize) -> char {
    let bit_0_count = total - bit_1_count;
    if bit_1_count >= bit_0_count { '0' } else { '1' }
}

fn get_rate(bits: &Vec<usize>, total: usize, filter: BitFilter) -> usize {
    usize::from_str_radix(
        &bits.iter().map(|&bit_1_count| filter(bit_1_count, total)).collect::<String>(),
        2
    ).expect("Error converting from binary in string")
}

fn longest_bits_number(numbers: &Vec<Vec<char>>) -> usize {
    numbers.iter().map(|number| number.len()).max().unwrap_or(0)
}

fn total_1s_in_each_position(numbers: &Vec<Vec<char>>) -> Vec<usize> {
    let mut bits = Vec::new();
    for _ in 0..(longest_bits_number(&numbers) - bits.len()) { bits.push(0) }
    for position in 0..bits.len() {
        bits[position] = total_1s_in_position(&numbers, position);
    }
    return bits;
}

fn total_1s_in_position(numbers: &Vec<Vec<char>>, position: usize) -> usize {
    numbers.iter().map(|number| if number[position] == '1' { 1 } else { 0 }).sum()
}

fn filter_numbers_by_position(numbers: &Vec<Vec<char>>, position: usize, filter: BitFilter) -> Vec<Vec<char>> {
    let total_1s_in_position = total_1s_in_position(&numbers, position);
    let bit_value_to_filter = filter(total_1s_in_position, numbers.len());
    return numbers.iter().filter(|&number| number[position] == bit_value_to_filter).map(|n| n.clone()).collect();
}

fn filter_position_by_position(numbers: &Vec<Vec<char>>, filter: BitFilter) -> usize {
    let max_length = longest_bits_number(&numbers);
    let mut numbers = numbers.clone();
    for position in 0..max_length {
        if numbers.len() == 1 { break }
        numbers = filter_numbers_by_position(&numbers, position, filter);
    }
    return usize::from_str_radix(
        &numbers[0].iter().collect::<String>(),
        2
    ).expect("Error converting from binary in string");
}

fn main() -> io::Result<()> {
    let numbers: Vec<Vec<char>> = Opt::from_args().lines()?.map(|line| line.chars().collect::<Vec<char>>()).collect();
    let bits = total_1s_in_each_position(&numbers);
    let total = numbers.len();
    let gamma_rate = get_rate(&bits, total, gamma_bit);
    let epsilon_rate = get_rate(&bits, total, epsilon_bit);
    println!("{}", gamma_rate * epsilon_rate);
    let oxygen_generator_rating = filter_position_by_position(&numbers, gamma_bit);
    let co2_scrubber_rating = filter_position_by_position(&numbers, epsilon_bit);
    println!("{}", oxygen_generator_rating * co2_scrubber_rating);
    return Ok(());
}
