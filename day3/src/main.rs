use regex::Regex;
use std::fs;

#[derive(Debug, PartialEq)]
struct Pair(u32, u32);

fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn find_mul_strings(line: &str) -> Vec<Pair> {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut matches: Vec<Pair> = vec![];

    for (_, [lh, rh]) in mul_re.captures_iter(line).map(|c| c.extract()) {
        matches.push(Pair(lh.parse().unwrap(), rh.parse().unwrap()));
    }
    matches
}

fn calc_total_product(pairs: Vec<Pair>) -> u32 {
    pairs.iter().map(|p| p.0 * p.1).sum()
}

fn main() {
    let input = read_input("input.txt");
    let total: u32 = input
        .lines()
        .map(find_mul_strings)
        .map(calc_total_product)
        .sum();
    println!("{:?}", total);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_input() {
        let input = read_input("testInput.txt");
        assert_eq!(
            input,
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))\r\n"
        );
    }

    #[test]
    fn test_find_mul_strings() {
        let matches = find_mul_strings(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))\r\n",
        );
        let pairs = Vec::from([Pair(2, 4), Pair(5, 5), Pair(11, 8), Pair(8, 5)]);
        assert_eq!(matches, pairs)
    }

    #[test]
    fn test_calc_total_product() {
        let pairs = Vec::from([Pair(2, 4), Pair(5, 5), Pair(11, 8), Pair(8, 5)]);
        let total_product = calc_total_product(pairs);

        assert_eq!(8 + 25 + 88 + 40, total_product)
    }
}
