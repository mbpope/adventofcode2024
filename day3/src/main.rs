use regex::Regex;
use std::fs;

fn main() {
    part1();    
}

fn part1() {
    let input = fs::read_to_string("data/input.txt").expect("something's wrong with the input file");
    
    let RE: Regex = Regex::new(r"mul\([0-9]{1,},[0-9]{1,}\)").unwrap(); // I'm a regex god (I'm not this wasn't a hard one)
    let mul_list: Vec<_> = RE.find_iter(&input).map(|m| {
        let total_str = m.as_str();
        let nums: &str = &total_str[4..total_str.len()-1];
        nums.split(",").map(|num_string| num_string.parse::<i32>().expect("something went horribly wrong")).collect::<Vec<_>>()
    }).collect();
    
    let mut total = 0;
    for i in 0..mul_list.len() {
        total += mul_list[i][0] * mul_list[i][1];
    }

    println!("The total for part 1 is: {}", total);
}