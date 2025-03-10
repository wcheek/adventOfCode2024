use std::{fs, io::Error};

#[derive(Debug, Ord, PartialOrd, Eq)]
pub struct LocID(u32);

impl From<&str> for LocID {
    fn from(value: &str) -> Self {
        LocID(value.parse().unwrap())
    }
}

impl PartialEq for LocID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

fn open_file(path: &str) -> Result<String, Error> {
    fs::read_to_string(path)
}

fn get_num_arrays(file: String) -> [Vec<LocID>; 2] {
    let mut num_array_one: Vec<LocID> = Vec::new();
    let mut num_array_two: Vec<LocID> = Vec::new();

    for line in file.lines() {
        let mut split_line = line.split_whitespace();
        num_array_one.push(split_line.next().unwrap().into());
        num_array_two.push(split_line.next().unwrap().into());
    }
    [num_array_one, num_array_two]
}

fn sort_num_array(mut arr: Vec<LocID>) -> Vec<LocID> {
    arr.sort();
    arr
}

fn find_differences(num_arrays: [Vec<LocID>; 2]) -> u32 {
    let mut difference = 0;
    let [array_one, array_two] = num_arrays;
    for (ind, n1) in array_one.iter().enumerate() {
        if n1.0 > array_two[ind].0 {
            difference += n1.0 - array_two[ind].0;
        } else {
            difference += array_two[ind].0 - n1.0;
        }
    }
    difference
}

fn main() {
    let file = open_file("input.txt").unwrap();

    let [num_array_one, num_array_two] = get_num_arrays(file);
    let sorted_num_array_one = sort_num_array(num_array_one);
    let sorted_num_array_two = sort_num_array(num_array_two);
    println!(
        "{}",
        find_differences([sorted_num_array_one, sorted_num_array_two])
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_file() {
        let test_input = open_file("testInput.txt").unwrap();
        assert_eq!(
            test_input,
            "3   4\r\n4   3\r\n2   5\r\n1   3\r\n3   9\r\n3   3\r\n"
        )
    }

    #[test]
    fn test_get_num_arrays() {
        let test_input = open_file("testInput.txt").unwrap();
        let [num_array_one, num_array_two] = get_num_arrays(test_input);

        assert_eq!(
            num_array_one,
            [LocID(3), LocID(4), LocID(2), LocID(1), LocID(3), LocID(3),]
        );
        assert_eq!(
            num_array_two,
            [LocID(4), LocID(3), LocID(5), LocID(3), LocID(9), LocID(3),]
        )
    }

    #[test]
    fn test_sort_num_array() {
        let num_array_one = Vec::from([LocID(3), LocID(4), LocID(2), LocID(1), LocID(3), LocID(3)]);
        let sorted_array = sort_num_array(num_array_one);

        assert_eq!(
            sorted_array,
            [LocID(1), LocID(2), LocID(3), LocID(3), LocID(3), LocID(4),]
        )
    }

    #[test]
    fn set_find_differences() {
        let array_one = Vec::from([LocID(1), LocID(2), LocID(3), LocID(3), LocID(3), LocID(4)]);
        let array_two = Vec::from([LocID(2), LocID(3), LocID(4), LocID(5), LocID(6), LocID(7)]);

        let differences = find_differences([array_one, array_two]);

        assert_eq!(11, differences)
    }
}
