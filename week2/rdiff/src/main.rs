use grid::Grid; // For lcs()
use std::cmp;
use std::env;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()
use std::process;

pub mod grid;

/// Reads the file at the supplied path, and returns a vector of strings.
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    // Be sure to delete the #[allow(unused)] line above
    let file = File::open(filename)?;
    let mut vec = Vec::<String>::new();
    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        vec.push(line_str);
    }
    Ok(vec)
}

fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
    // Note: Feel free to use unwrap() in this code, as long as you're basically certain it'll
    // never happen. Conceptually, unwrap() is justified here, because there's not really any error
    // condition you're watching out for (i.e. as long as your code is written correctly, nothing
    // external can go wrong that we would want to handle in higher-level functions). The unwrap()
    // calls act like having asserts in C code, i.e. as guards against programming error.
    // Be sure to delete the #[allow(unused)] line above
    let len1 = seq1.len();
    let len2 = seq2.len();
    let mut grid = Grid::new(len1 + 1, len2 + 1);
    for i in 0..=len1 {
        grid.set(i, 0, 0).unwrap();
    }
    for i in 0..=len2 {
        grid.set(0, i, 0).unwrap();
    }
    for i in 1..=len1 {
        for j in 1..=len2 {
            if seq1[i - 1].eq(&seq2[j - 1]) {
                grid.set(i, j, grid.get(i - 1, j - 1).unwrap() + 1).unwrap();
            } else {
                grid.set(
                    i,
                    j,
                    cmp::max(grid.get(i - 1, j).unwrap(), grid.get(i, j - 1).unwrap()),
                )
                .unwrap();
            }
        }
    }
    grid
}

fn print_diff(lcs_table: &Grid, lines1: &Vec<String>, lines2: &Vec<String>, i: usize, j: usize) {
    // Be sure to delete the #[allow(unused)] line above
    let len1 = lines1.len();
    let len2 = lines2.len();
    let mut index1 = i;
    let mut index2 = j;
    let mut vec = Vec::<String>::new();
    while index1 > 0 || index2 > 0 {
        if index1 > 0 && index2 > 0 && lines1[index1 - 1].eq(&lines2[index2 - 1]) {
            vec.push(format!("{}{}", " ", lines1[index1 - 1]));
            index1 -= 1;
            index2 -= 1;
        } else if index2 > 0
            && (index1 == 0
                || lcs_table.get(index1, index2 - 1).unwrap()
                    >= lcs_table.get(index1 - 1, index2).unwrap())
        {
            vec.push(format!(">{}", lines2[index2 - 1]));
            index2 -= 1
        } else if index1 > 0
            && (index2 == 0
                || lcs_table.get(index1 - 1, index2).unwrap()
                    > lcs_table.get(index1, index2 - 1).unwrap())
        {
            vec.push(format!("<{}", lines1[index1 - 1]));
            index1 -= 1;
        }
    }
    vec.push(String::from(""));
    vec.reverse();
    for lines in vec.iter() {
        println!("{}", lines);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename1 = &args[1];
    let filename2 = &args[2];

    // Be sure to delete the #[allow(unused)] line above
    let lines1 = match read_file_lines(filename1) {
        Ok(line) => line,
        Err(..) => {
            println!("addr {} is invalid", filename1);
            process::exit(1);
        }
    };
    let lines2 = match read_file_lines(filename2) {
        Ok(line) => line,
        Err(..) => {
            println!("addr {} is invalid", filename2);
            process::exit(1);
        }
    };
    let grid = lcs(&lines1, &lines2);
    print_diff(&grid, &lines1, &lines2, lines1.len(), lines2.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_lines() {
        let lines_result = read_file_lines(&String::from("handout-a.txt"));
        assert!(lines_result.is_ok());
        let lines = lines_result.unwrap();
        assert_eq!(lines.len(), 8);
        assert_eq!(
            lines[0],
            "This week's exercises will continue easing you into Rust and will feature some"
        );
    }

    #[test]
    fn test_lcs() {
        let mut expected = Grid::new(5, 4);
        expected.set(1, 1, 1).unwrap();
        expected.set(1, 2, 1).unwrap();
        expected.set(1, 3, 1).unwrap();
        expected.set(2, 1, 1).unwrap();
        expected.set(2, 2, 1).unwrap();
        expected.set(2, 3, 2).unwrap();
        expected.set(3, 1, 1).unwrap();
        expected.set(3, 2, 1).unwrap();
        expected.set(3, 3, 2).unwrap();
        expected.set(4, 1, 1).unwrap();
        expected.set(4, 2, 2).unwrap();
        expected.set(4, 3, 2).unwrap();

        println!("Expected:");
        expected.display();
        let result = lcs(
            &"abcd".chars().map(|c| c.to_string()).collect(),
            &"adb".chars().map(|c| c.to_string()).collect(),
        );
        println!("Got:");
        result.display();
        assert_eq!(result.size(), expected.size());
        for row in 0..expected.size().0 {
            for col in 0..expected.size().1 {
                assert_eq!(result.get(row, col), expected.get(row, col));
            }
        }
    }
}
