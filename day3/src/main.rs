use regex::Regex;
use std::fs;

fn main() {
    part1(); 
    part2();  
}

fn part1() {
    let input = fs::read_to_string("data/input.txt").expect("something's wrong with the input file");
    
    let re: Regex = Regex::new(r"mul\([0-9]{1,},[0-9]{1,}\)").unwrap(); // I'm a regex god (I'm not this wasn't a hard one)
    let mul_list: Vec<_> = re.find_iter(&input).map(|m| {
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

fn part2() {
    let input = fs::read_to_string("data/input.txt").expect("something's wrong with the input file");
    let re: Regex = Regex::new(r"(mul\([0-9]+,[0-9]+\))|(do\(\))|(don't\(\))").unwrap();
    let uncorrupted_list: Vec<_> = re.find_iter(&input).map(|m| m.as_str()).collect();

    let mut valid_mul = true;
    let mut total = 0;
    for i in 0..uncorrupted_list.len() {
        if uncorrupted_list[i] == "do()" {
            valid_mul = true;
        } else if uncorrupted_list[i] == "don't()" {
            valid_mul = false;
        } else if valid_mul {
            let nums = &uncorrupted_list[i][4..uncorrupted_list[i].len() - 1];
            let num_arr = nums.split(",")
                .map(|num_string| num_string.parse::<i32>().expect("something has gone terribly wrong"))
                .collect::<Vec<_>>();
            total += num_arr[0] * num_arr[1];
        }
    }

    println!("The answer for part 2 is: {}", total);
}