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
        let safe = validate_report_data_with_dampener(&mut report_data);
        if !safe {
            println!("{:?}", data);
        }
        Report {
            data: report_data,
            safe,
        }
    }
}

fn validate_report_data_with_dampener(report_data: &mut Vec<u32>) -> bool {
    let mut num_problems = 0;
    let mut problem_option = validate_report_data(report_data);
    // println!("{:?}", report_data);
    // println!("{:?}", problem_option);
    while let Some(problem_ind) = problem_option {
        report_data.remove(problem_ind);
        num_problems += 1;
        if num_problems > 1 {
            return false;
        }
        problem_option = validate_report_data(report_data);
        // println!("{:?}", report_data);
        // println!("{:?}", problem_option);
    }
    true
}

fn validate_report_data(data: &[u32]) -> Option<usize> {
    let direction = data[1].cmp(&data[0]);

    for i in 1..data.len() {
        if (direction == Ordering::Equal)
            | (data[i].abs_diff(data[i - 1]) > 3)
            | (data[i].cmp(&data[i - 1]) != direction)
        {
            return Some(i - 1);
        }
    }
    None
}

fn open_file(path: &str) -> Result<String, Error> {
    fs::read_to_string(path)
}

fn get_reports(path: &str) -> Vec<Report> {
    let mut reports: Vec<Report> = Vec::new();
    for line in open_file(path).unwrap().lines() {
        reports.push(Report::new(line));
    }
    println!("{}", reports.len());
    reports
}

fn count_num_safe_reports(reports: Vec<Report>) -> usize {
    reports.iter().filter(|report| report.safe).count()
}

fn main() {
    println!("{}", count_num_safe_reports(get_reports("input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_get_report() {
    //     assert_eq!(
    //         Report::new("7 6 4 2 1"),
    //         Report {
    //             data: Vec::from([7, 6, 4, 2, 1]),
    //             safe: true
    //         }
    //     );
    //     assert_eq!(
    //         Report::new("7 6 4 2 1 0"),
    //         Report {
    //             data: Vec::from([7, 6, 4, 2, 1, 0]),
    //             safe: true
    //         }
    //     );
    // }

    #[test]
    fn test_validate_report_data() {
        let mut data = Vec::from([7, 6, 4, 2, 1]);
        let safe = validate_report_data_with_dampener(&mut data);
        assert!(safe);

        let mut data = Vec::from([1, 2, 7, 8, 9]);
        let safe = validate_report_data_with_dampener(&mut data);
        assert!(!safe);

        let mut data = Vec::from([9, 7, 6, 2, 1]);
        let safe = validate_report_data_with_dampener(&mut data);
        assert!(!safe);

        let mut data = Vec::from([1, 3, 2, 4, 5]);
        let safe = validate_report_data_with_dampener(&mut data);
        assert!(safe);

        let mut data = Vec::from([8, 6, 4, 4, 1]);
        let safe = validate_report_data_with_dampener(&mut data);
        assert!(safe);

        let mut data = Vec::from([1, 3, 6, 7, 9]);
        let safe = validate_report_data_with_dampener(&mut data);
        assert!(safe);

        // Additional test cases..
        let mut data = Vec::from([15, 21, 24, 25, 26, 30, 30]);
        let safe = validate_report_data_with_dampener(&mut data);
        assert!(!safe);

        let mut data = Vec::from([20, 22, 24, 25, 26, 28, 30]);
        let safe = validate_report_data_with_dampener(&mut data);
        assert!(safe);

        let mut data = Vec::from([71, 72, 70, 71, 73, 76, 78, 78]);
        let safe = validate_report_data_with_dampener(&mut data);
        assert!(!safe);

        let mut data = Vec::from([60, 59, 58, 55, 53, 50, 48, 47]);
        let safe = validate_report_data_with_dampener(&mut data);
        assert!(safe);

        let mut data = Vec::from([60, 57, 58, 56, 53, 50, 48, 47]);
        let safe = validate_report_data_with_dampener(&mut data);
        assert!(safe);

        let mut data = Vec::from([34, 30, 29, 27, 25, 22, 24]);
        let safe = validate_report_data_with_dampener(&mut data);
        assert!(!safe);
    }

    #[test]
    fn test_count_num_safe_reports() {
        let num_safe_reports = count_num_safe_reports(get_reports("testInput.txt"));
        assert_eq!(num_safe_reports, 4);
    }
}
