use regex::Regex;
use std::fs;

#[derive(Debug, PartialEq)]
struct Pair(u32, u32);

#[derive(Debug, PartialEq)]
struct Command {
    pair: Pair,
    do_mul: bool,
}

fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn find_mul_strings(line: &str) -> Vec<Command> {
    let mul_re =
        Regex::new(r"mul\((?<lh>\d{1,3}),(?<rh>\d{1,3})\)|(?<cmd>do(?:n't)*\(\))").unwrap();

    let mut matches: Vec<Command> = vec![];

    let mut instruction = true;

    for cap in mul_re.captures_iter(line) {
        if let Some(cmd) = cap.name("cmd") {
            instruction = cmd.as_str() == "do()";
        } else {
            matches.push(Command {
                pair: Pair(
                    cap.name("lh").unwrap().as_str().parse().unwrap(),
                    cap.name("rh").unwrap().as_str().parse().unwrap(),
                ),
                do_mul: instruction,
            })
        }
    }
    println!("{:?}", matches);
    matches
}

fn calc_total_product(pairs: Vec<Command>) -> u32 {
    pairs
        .iter()
        .map(|p| if p.do_mul { p.pair.0 * p.pair.1 } else { 0 })
        .sum()
}

fn main() {
    let input = read_input("input.txt");
    // input.lines().map(find_mul_strings);
    let all_lines = input.lines().collect::<String>();
    let total = calc_total_product(find_mul_strings(&all_lines));
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
        let result = find_mul_strings(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))\r\n",
        );
        let expected = Vec::from([
            Command {
                pair: Pair(2, 4),
                do_mul: true,
            },
            Command {
                pair: Pair(5, 5),
                do_mul: false,
            },
            Command {
                pair: Pair(11, 8),
                do_mul: false,
            },
            Command {
                pair: Pair(8, 5),
                do_mul: true,
            },
        ]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_calc_total_product() {
        let pairs = Vec::from([
            Command {
                pair: Pair(2, 4),
                do_mul: true,
            },
            Command {
                pair: Pair(5, 5),
                do_mul: false,
            },
            Command {
                pair: Pair(11, 8),
                do_mul: false,
            },
            Command {
                pair: Pair(8, 5),
                do_mul: true,
            },
        ]);
        let total_product = calc_total_product(pairs);

        assert_eq!(8 + 40, total_product)
    }
}
