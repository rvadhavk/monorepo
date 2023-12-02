use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./1.txt").unwrap();

    let sum1: u32 = input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first_digit = digits.next().unwrap();
            let last_digit = digits.last().unwrap_or(first_digit);
            return 10 * first_digit + last_digit;
        })
        .sum();
    println!("{}", sum1);

    let digit_strings = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let sum2: usize = input
        .lines()
        .map(|line| {
            let suffixes = (0..line.len()).map(|i| line.get(i..).unwrap());
            let mut digits = suffixes.filter_map(|suffix| {
                digit_strings
                    .iter()
                    .position(|&s| suffix.starts_with(s))
                    .or(suffix
                        .chars()
                        .next()
                        .unwrap()
                        .to_digit(10)
                        .and_then(|d| usize::try_from(d).ok()))
            });
            let first_digit: usize = digits.next().unwrap();
            let last_digit: usize = digits.last().unwrap_or(first_digit);
            return 10 * first_digit + last_digit;
        })
        .sum();
    println!("{}", sum2);
}
