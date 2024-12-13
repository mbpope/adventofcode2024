use std::fs;
use std::str::FromStr;

// this is a true testament to the fact that I have absolutely no idea what I'm doing

fn main() {
    let input: Vec<String> = read_lines("data/input.txt");
    let reports: Vec<Vec<i32>> = input.iter()
        .map(|report: &String| -> Vec<i32> {
            report.split(" ").map(|num| i32::from_str(num).unwrap()).collect()
        }).collect();
    
    // there's only 1000 data points. calm down. we're using clone().
    part1(reports.clone());
    part2(reports.clone());
}

fn part1(reports: Vec<Vec<i32>>) {
    let mut safe_reports = 0;
    for i in 0..reports.len() {
        // this could easily be optimised...but there's only 1000 data points so I don't care
        if all_increase_or_decrease(&reports[i]).safe && differ_by_little(&reports[i]).safe {
            safe_reports += 1;
        }
    }
    println!("Answer to Pt. 1 is: {}", safe_reports);
}

fn part2(reports: Vec<Vec<i32>>) {
    let mut safe_reports = 0;
    for i in 0..reports.len() {
        if all_increase_or_decrease(&reports[i]).safe && differ_by_little(&reports[i]).safe {
            safe_reports += 1;
        } else {
            for datum_index in 0..reports[i].len() {
                let mut report = reports[i].clone();
                report.remove(datum_index);
                if all_increase_or_decrease(&report).safe && differ_by_little(&report).safe {
                    safe_reports += 1;
                    break
                }       
            }
        }
    }
    println!("Answer to Pt. 2 is: {}", safe_reports);
}

struct SafetyReport {
    safe: bool,
    unsafe_pos: Option<usize>,
}

fn all_increase_or_decrease(arr: &Vec<i32>) -> SafetyReport {
    enum Direction {
        Up,
        Down,
    }

    let presumed_dir = if arr[0] > arr[1] {
        Direction::Down
    } else if arr[0] < arr[1] {
        Direction::Up
    } else {
        return SafetyReport {
            safe: false,
            unsafe_pos: Some(0),
        }
    };

    for i in 1..arr.len() {
        match presumed_dir {
            Direction::Up => {
                if arr[i] <= arr[i-1] {
                    return SafetyReport {
                        safe: false,
                        unsafe_pos: Some(i),
                    }
                }
                ()
            },
            Direction::Down => {
                if arr[i] >= arr[i-1] {
                    return SafetyReport {
                        safe: false,
                        unsafe_pos: Some(i),
                    }
                }
                ()
            },
        }
    }
    
    SafetyReport {
        safe: true,
        unsafe_pos: None,
    }
}

fn differ_by_little(arr: &Vec<i32>) -> SafetyReport {
    for i in 1..arr.len() {
        let diff = arr[i] - arr[i-1];
        if diff.abs() > 3 {
            return SafetyReport {
                safe: false,
                unsafe_pos: Some(i),
            }
        }
    }
    
    SafetyReport {
        safe: true,
        unsafe_pos: None,
    }
}

// shamelessly stolen from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename) 
        .unwrap()  
        .lines()  
        .map(String::from)  
        .collect()  
}
