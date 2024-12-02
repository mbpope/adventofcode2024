use std::fs;
use std::str::FromStr;

fn main() {
    let locations = read_lines("data/input.txt");
    let mut locations_a: Vec<i32> = locations.iter().map(|x| i32::from_str(&x[0..5]).unwrap()).collect();
    let mut locations_b: Vec<i32> = locations.iter().map(|x| i32::from_str(&x[8..13]).unwrap()).collect();
    locations_a.sort();
    locations_b.sort();

    let mut res:i32 = 0;
    for i in 0..locations.len() {
        let diff = locations_a[i] - locations_b[i];
        res = res + diff.abs();
    }

    println!("The grand total is: {}", res);
}

fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}
