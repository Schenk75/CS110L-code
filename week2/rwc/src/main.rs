use std::env;
use std::process;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()

fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let mut res = vec![];

    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        res.push(line_str);
    }
    Ok(res)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)

    let lines = read_file_lines(filename).expect("Cannot open file");
    println!("Number of lines: {}", lines.len());

    let mut count_words: u32 = 0;
    let mut count_ch: u32 = 0;

    for line in lines.iter() {
        for word in line.split(" ") {
            count_words += 1;
            count_ch += word.len() as u32;
        }
    }

    println!("Number of words: {}", count_words);
    // not count blanks
    println!("Number of characters: {}", count_ch);
}
