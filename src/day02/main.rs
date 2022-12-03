use std::{fs::File, io::{BufReader, Read}};

fn main() -> std::io::Result<()>{
    let file = File::open("src/day01/input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    
    Ok(())
}