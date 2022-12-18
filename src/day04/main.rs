use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() -> std::io::Result<()> {
    let file = File::open("src/day04/input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let sum = contents.lines().fold(0, |acc, line| {
        if let [first_elf, second_elf] = line.split(',').collect::<Vec<&str>>()[..] {
            let first_section: Vec<&str> = first_elf.split('-').collect();
            let second_section: Vec<&str> = second_elf.split('-').collect();
            let (first_start, first_end) = get_indexes(&first_section);
            let (second_start, second_end) = get_indexes(&second_section);

            let first_substring = get_substring((&first_start, &first_end));
            let second_substring = get_substring((&second_start, &second_end));
            if first_start >= second_start
                && first_end <= second_end
                && second_substring.contains(&first_substring)
            {
                acc + 1
            } else if second_start >= first_start
                && second_end <= first_end
                && first_substring.contains(&second_substring)
            {
                acc + 1
            } else {
                acc
            }
        } else {
            acc
        }
    });
    println!("{}", sum);

    let sum_v2 = contents.lines().fold(0, |acc, line| {
        if let [first_elf, second_elf] = line.split(',').collect::<Vec<&str>>()[..] {
            let first_section: Vec<&str> = first_elf.split('-').collect();
            let second_section: Vec<&str> = second_elf.split('-').collect();
            let (first_start, first_end) = get_indexes(&first_section);
            let (second_start, second_end) = get_indexes(&second_section);
            
            if first_start <= second_start && first_end >= second_start {
                acc + 1
            } else if second_start <= first_start && second_end >= first_start {
                acc + 1
            } else {
                acc
            }
        } else {
            acc
        }
    });
    println!("{}", sum_v2);
    Ok(())
}

fn get_indexes(section: &Vec<&str>) -> (usize, usize) {
    if let [first, last] = section[..] {
        (first.parse().unwrap(), last.parse().unwrap())
    } else {
        panic!("The impossible happened")
    }
}

fn get_substring(tuple: (&usize, &usize)) -> String {
    (*tuple.0..=*tuple.1).fold("".to_owned(), |acc, index| acc + &index.to_string())
}
