use std::fs;

fn main() {

    let file_contents = fs::read_to_string("Input_2.txt").expect("File read error!");
    let mut my_total_score_part_1 : u32 = 0;
    let mut my_total_score_part_2 : u32 = 0;
    
    for line in file_contents.lines() {
    
        let opponent_shape = line.chars().nth(0).unwrap() as u32 - 'A' as u32 + 1;
        let mut my_shape = line.chars().nth(2).unwrap() as u32 - 'X' as u32 + 1;

        my_total_score_part_1 += my_shape;
        
        // Part 1 logic
        if (opponent_shape % 3) + 1 == my_shape         // I'm the winner
        {
            my_total_score_part_1 += 6;
        }
        else if (my_shape % 3) + 1 == opponent_shape    // Opponent is the winner
        {
        }
        else                                            // Draw
        {
            my_total_score_part_1 += 3;
        }

        // Part 2 logic
        if my_shape == 1  // To lose, pick a shape above opponent's
        {
            my_shape = ((opponent_shape + 1) % 3) + 1;
        }
        else if my_shape == 2 // To draw, pick the same shape as opponent
        {
            my_shape = opponent_shape;
            my_total_score_part_2 += 3;
        }
        else if my_shape == 3 // To win, pick a shape below the opponent
        {
            my_shape = (opponent_shape % 3) + 1;
            my_total_score_part_2 += 6;
        }
        else
        {
            println!("Wtf...");
        }

        my_total_score_part_2 += my_shape;
    }
    
    println!("Part 1: {}", my_total_score_part_1);
    println!("Part 2: {}", my_total_score_part_2);
}