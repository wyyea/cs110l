use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process; // For read_file_lines() // For read_file_lines()

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(..) => {
            println!("address {} is invalid", filename);
            process::exit(1);
        }
    };
    let mut lc = 0;
    let mut wc = 0;
    let mut cc = 0;
    for lines in io::BufReader::new(file).lines() {
        match lines {
            Err(..) => {
                println!("read_line error");
                process::exit(1);
            }
            Ok(line) => {
                lc += 1;
                let word_list: Vec<&str> = line.split(' ').collect();
                wc += word_list.len();
                cc += word_list.len();
                for &word in word_list.iter() {
                    cc += word.len();
                }
            }
        }
    }
    println!("lines: {}", lc);
    println!("words: {}", wc);
    println!("chars: {}", cc);
}
