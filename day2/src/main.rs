use std::{cmp::Ordering, fs, io::Error};

#[derive(Debug, PartialEq)]
struct Report {
    data: Vec<u32>,
    safe: bool,
}

// impl From<&str> for Report {
//     fn from(value: &str) -> Self {
//         let mut report: Vec<u32> = Vec::new();
//         for n in value.split_whitespace() {
//             report.push(n.parse().unwrap());
//         }
//         Report {
//             data: report,
//             safe: Self::validate_report(),
//         }
//     }
// }

impl Report {
    fn new(data: &str) -> Self {
        let mut report_data: Vec<u32> = Vec::new();
        for n in data.split_whitespace() {
            report_data.push(n.parse().unwrap());
        }
        let safe = validate_report_data(&report_data);
        Report {
            data: report_data,
            safe,
        }
    }
}

fn validate_report_data(data: &[u32]) -> bool {
    // Check for monotonicity and spacing..
    if data.len() <= 2 {
        return true;
    }

    let direction = data[1].cmp(&data[0]);
    if data[1].abs_diff(data[0]) > 3 {
        return false;
    }
    for i in 2..data.len() {
        if direction == Ordering::Equal {
            return false;
        }
        if data[i].abs_diff(data[i - 1]) > 3 {
            return false;
        }
        if breaks_direction(direction, data[i], data[i - 1]) {
            return false;
        }
    }
    true
}

fn breaks_direction(direction: Ordering, current: u32, previous: u32) -> bool {
    let difference = current.cmp(&previous);
    if direction != difference {
        return true;
    }
    false
}

fn open_file(path: &str) -> Result<String, Error> {
    fs::read_to_string(path)
}

fn main() {
    let mut reports: Vec<Report> = Vec::new();
    for line in open_file("input.txt").unwrap().lines() {
        reports.push(Report::new(line));
    }
    println!("{}", reports.iter().filter(|report| report.safe).count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_report() {
        assert_eq!(
            Report::new("7 6 4 2 1"),
            Report {
                data: Vec::from([7, 6, 4, 2, 1]),
                safe: true
            }
        );
        assert_eq!(
            Report::new("7 6 4 2 1 0"),
            Report {
                data: Vec::from([7, 6, 4, 2, 1, 0]),
                safe: true
            }
        );
    }

    #[test]
    fn test_validate_report_data() {
        let data = Vec::from([7, 6, 4, 2, 1, 0]);
        let safe = validate_report_data(&data);
        assert!(safe);

        let data = Vec::from([7, 6, 7, 2, 1, 0]);
        let safe = validate_report_data(&data);
        assert!(!safe);

        let mut data = Vec::from([7, 6, 4, 2, 1, 0]);
        data.reverse();
        let safe = validate_report_data(&data);
        assert!(safe);

        let mut data = Vec::from([7, 6, 7, 2, 1, 0]);
        data.reverse();
        let safe = validate_report_data(&data);
        assert!(!safe);

        let data = Vec::from([7, 6, 6, 2, 1, 0]);
        let safe = validate_report_data(&data);
        assert!(!safe);

        let data = Vec::from([11, 6, 5, 2, 1, 0]);
        let safe = validate_report_data(&data);
        assert!(!safe);

        let data = Vec::from([9, 8, 3, 2, 1, 0]);
        let safe = validate_report_data(&data);
        assert!(!safe);
    }
}
