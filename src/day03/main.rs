use std::{
    fs::File,
    io::{BufReader, Read}
};

fn main() -> std::io::Result<()> {
    let file = File::open("src/day03/input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let sum = contents.lines().fold(0, |acc, line| {
        let (first_item, second_item) = line.split_at(line.len()/2);
        if let Some(common_item) = first_item.find(|character|{ second_item.contains(character)}) {
            priority(first_item.chars().collect::<Vec<char>>()[common_item]) + acc
        }
        else {
            acc
        }
    });
    println!("{}", sum);

    
    // part two
    
    let mut groups_vec = vec![vec![]];
    
    contents.lines().enumerate().for_each(|(index, line)| {

        let mut group = index - (index % 3);
        group =  if group > 0 { group / 3} else {group};
        let group_element  = index % 3;
        // println!("group: {}  group element: {}", group, group_element);
        if group_element == 0 && group > 0 {
            groups_vec.push(vec![])
        }
        groups_vec[group].push(line)
    });
    
    let group_sum = groups_vec.into_iter().fold(0, |acc, group| {
        if let [first_item, second_item, third_item] = group[0..=2] {

            if let Some(common_item) = first_item.find(|character|{ second_item.contains(character) && third_item.contains(character)}) {
                priority(first_item.chars().collect::<Vec<char>>()[common_item]) + acc
            } else {
                acc
            }
        } else {
            acc
        }
        
    });
    println!("{}", group_sum);

    Ok(())
}

fn priority(letter: char) -> u32 {

    if let ('a'..='z') = letter {
        ('a'..='z').position(|character| { character == letter}).unwrap() as u32 + 1
    } else {
        ('A'..='Z').position(|character| { character == letter}).unwrap() as u32 + 27
    } 
}