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
    let [left, right] = num_arrays;
    for (ind, n1) in left.iter().enumerate() {
        if n1.0 > right[ind].0 {
            difference += n1.0 - right[ind].0;
        } else {
            difference += right[ind].0 - n1.0;
        }
    }
    difference
}

fn find_num_occurrences(arr: &Vec<LocID>, num_to_find: &LocID) -> u32 {
    let mut num_occurrences = 0;

    for num in arr {
        if num == num_to_find {
            num_occurrences += 1;
        }
    }
    num_occurrences
}

fn find_similarity(num_arrays: [Vec<LocID>; 2]) -> u32 {
    let mut similarity = 0;
    let [left, right] = num_arrays;
    for n1 in left.iter() {
        let num_occurrences_for_num = find_num_occurrences(&right, n1);
        similarity += n1.0 * num_occurrences_for_num;
    }

    similarity
}

fn main() {
    let file = open_file("input.txt").unwrap();
    let [left, right] = get_num_arrays(file);
    let left = sort_num_array(left);
    let right = sort_num_array(right);
    // println!("{}", find_differences([left, right]))
    println!("{}", find_similarity([left, right]))
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
        let [left, right] = get_num_arrays(test_input);

        assert_eq!(
            left,
            [LocID(3), LocID(4), LocID(2), LocID(1), LocID(3), LocID(3),]
        );
        assert_eq!(
            right,
            [LocID(4), LocID(3), LocID(5), LocID(3), LocID(9), LocID(3),]
        )
    }

    #[test]
    fn test_sort_num_array() {
        let left = Vec::from([LocID(3), LocID(4), LocID(2), LocID(1), LocID(3), LocID(3)]);
        let sorted_array = sort_num_array(left);

        assert_eq!(
            sorted_array,
            [LocID(1), LocID(2), LocID(3), LocID(3), LocID(3), LocID(4),]
        )
    }

    #[test]
    fn test_find_differences() {
        let left = Vec::from([LocID(1), LocID(2), LocID(3), LocID(3), LocID(3), LocID(4)]);
        let right = Vec::from([LocID(2), LocID(3), LocID(4), LocID(5), LocID(6), LocID(7)]);

        let differences = find_differences([left, right]);

        assert_eq!(11, differences)
    }

    #[test]
    fn test_find_num_occurrences() {
        let arr = Vec::from([LocID(1), LocID(1), LocID(2), LocID(1), LocID(6), LocID(6)]);

        let num_occurrences = find_num_occurrences(&arr, &LocID(1));
        assert_eq!(3, num_occurrences);

        let num_occurrences = find_num_occurrences(&arr, &LocID(2));
        assert_eq!(1, num_occurrences);

        let num_occurrences = find_num_occurrences(&arr, &LocID(6));
        assert_eq!(2, num_occurrences);
    }

    #[test]
    fn test_find_similarities() {
        let left = Vec::from([LocID(1), LocID(2), LocID(3), LocID(3), LocID(3), LocID(4)]);
        let right = Vec::from([LocID(1), LocID(1), LocID(2), LocID(3), LocID(6), LocID(7)]);

        let similarity = find_similarity([left, right]);

        assert_eq!(2 + 2 + 3 + 3 + 3, similarity)
    }
}
