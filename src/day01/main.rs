use std::{fs::File, io::{BufReader, Read}};


fn main() -> std::io::Result<()>{
    let file = File::open("src/day01/input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let elves_calories: Vec<Vec<&str>> = contents.split("\n\n").collect::<Vec<&str>>().iter().map(|s| { s.lines().collect::<Vec<&str>>() }).collect();

    let max = elves_calories.iter().fold(0, |acc, calories| {
        let sum = calories.iter().map(|c| {
            c.parse::<i32>().unwrap()
        }).sum();
        if acc < sum {
            sum
        } else {
            acc
        }
    });
    let mut sum_of_calories = elves_calories.iter().map(|calories| {
        calories.iter().map(|c|{c.parse::<i32>().unwrap()}).sum()
    }).collect::<Vec<i32>>();
    sum_of_calories.sort();
    let top_3: i32 = sum_of_calories[sum_of_calories.len()-3..sum_of_calories.len()].iter().sum();
    println!("{}",max);
    println!("{}",top_3);

    Ok(())
}
