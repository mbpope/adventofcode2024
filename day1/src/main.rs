use std::fs;
use std::str::FromStr;

fn main() {
    let locations = read_lines("data/input.txt");
    let mut locations_a: Vec<i32> = locations.iter().map(|x| i32::from_str(&x[0..5]).unwrap()).collect();
    let mut locations_b: Vec<i32> = locations.iter().map(|x| i32::from_str(&x[8..13]).unwrap()).collect();
    locations_a.sort();
    locations_b.sort();
    
    part1(locations_a.clone(), locations_b.clone());
    part2(locations_a.clone(), locations_b.clone());
}

fn part1(locations_a: Vec<i32>, locations_b: Vec<i32>) {
    let mut res:i32 = 0;
    for i in 0..locations_a.len() {
        let diff = locations_a[i] - locations_b[i];
        res += diff.abs();
    }

    println!("Pt. 1: {}", res);
}

fn part2(locations_a: Vec<i32>, locations_b: Vec<i32>) {
    let loc_len = locations_a.len();
    let mut sim_score = 0;
    for i in 0..loc_len {
        let mut num_similarities = 0;
        for j in 0..loc_len {
            if locations_a[i] == locations_b[j] {
                num_similarities += 1;
            }
        }
        sim_score += locations_a[i] * num_similarities;
    }
    println!("Pt. 2: {}", sim_score);
}

// shamelessly stolen from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename) 
        .unwrap()  
        .lines()  
        .map(String::from)  
        .collect()  
}
