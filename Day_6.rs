use std::fs;

fn main() {

    let file_contents = fs::read_to_string("Input_6.txt").expect("File read error!");

    println!("Part 1: {}", (0..file_contents.len()).position(|i| (i..=i+3).all(|j| !file_contents[j+1..=i+3].contains(file_contents.chars().nth(j).unwrap()))).unwrap() + 4);
    println!("Part 2: {}", (0..file_contents.len()).position(|i| (i..=i+13).all(|j| !file_contents[j+1..=i+13].contains(file_contents.chars().nth(j).unwrap()))).unwrap() + 14);
}