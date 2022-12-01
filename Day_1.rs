use std::fs;

fn main() {

    let file_contents = fs::read_to_string("Input_1.txt").expect("File read error!");
    let mut greatest_sum : i32 = 0;
    let mut second_greatest_sum : i32 = 0;
    let mut third_greatest_sum : i32 = 0;
    let mut current_sum : i32 = 0;

    for line in file_contents.lines() {
        
        //Ensure data always ends with a newline for the last element
        if line.is_empty() {
            if current_sum > greatest_sum {
                third_greatest_sum = second_greatest_sum;
                second_greatest_sum = greatest_sum;
                greatest_sum = current_sum;
            }
            else if current_sum > second_greatest_sum {
                third_greatest_sum = second_greatest_sum;
                second_greatest_sum = current_sum;
            }
            else if current_sum > third_greatest_sum {
                third_greatest_sum = current_sum;
            }

            current_sum = 0;
        }
        else {
            current_sum += line.parse::<i32>().unwrap();
        }
    }

    println!("Part 1: {}", greatest_sum);
    println!("Part 2: {}", greatest_sum + second_greatest_sum + third_greatest_sum);
}