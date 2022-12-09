use std::fs;

fn main() {
    let file_contents = fs::read_to_string("Input_8.txt").expect("File read error!");

    let mut tree_array : Vec<Vec<u8>> = Vec::new();
    for line in file_contents.lines()
    {
        tree_array.push(Vec::new());
        for character in line.chars()
        {
            tree_array.last_mut().unwrap().push(character.to_digit(10).unwrap() as u8);
        }
    }

    let num_rows = tree_array[0].len();
    let num_cols = tree_array.len();
    let perimeter = ((num_rows + num_cols) * 2) - 4;
    let mut visible_trees = perimeter;

    let mut visibility_array    : Vec<Vec<bool>> = vec![vec![false; num_cols]; num_rows];
    let mut max_score = 0;

    for i in 1..num_rows-1
    {
        for j in 1..num_cols-1
        {   
            let mut visible = true;

            for k in 0..j
            {
                if tree_array[i][k] >= tree_array[i][j]
                {
                    visible = false;
                    break;
                }
            }

            if visible
            {
                visibility_array[i][j] = true;
                continue;
            }

            visible = true;

            for k in j + 1..num_cols
            {
                if tree_array[i][k] >= tree_array[i][j]
                {
                    visible = false;
                    break;
                }
            }

            if visible
            {
                visibility_array[i][j] = true;
                continue;
            }

            visible = true;

            for k in 0..i
            {
                if tree_array[k][j] >= tree_array[i][j]
                {
                    visible = false;
                    break;
                }
            }

            if visible
            {
                visibility_array[i][j] = true;
                continue;
            }

            visible = true;

            for k in i+1..num_rows
            {
                if tree_array[k][j] >= tree_array[i][j]
                {
                    visible = false;
                    break;
                }
            }

            if visible
            {
                visibility_array[i][j] = true;
                continue;
            }
        }
    }

    for i in 0..num_rows
    {
        for j in 0..num_cols
        {
            if visibility_array[i][j]
            {
                visible_trees += 1;
            }
        }
    }

    for i in 1..num_rows-1
    {
        for j in 1..num_cols-1
        {   
            let mut score = 1;

            for k in (0..j).rev()
            {
                if (tree_array[i][k] >= tree_array[i][j]) || (k == 0)
                {
                    score = score * (j - k);
                    break;
                }
            }
            
            for k in j + 1..num_cols
            {
                if (tree_array[i][k] >= tree_array[i][j]) || (k == (num_cols - 1))
                {
                    score = score * (k - j);
                    break;
                }
            }

            for k in (0..i).rev()
            {
                if (tree_array[k][j] >= tree_array[i][j]) || (k == 0)
                {
                    score = score * (i - k);
                    break;
                }
            }

            for k in i+1..num_rows
            {
                if (tree_array[k][j] >= tree_array[i][j]) || (k == (num_rows - 1))
                {
                    score = score * (k - i);
                    break;
                }
            }

            if score > max_score
            {
                max_score = score;
            }
        }
    }

    println!("Part 1: {}", visible_trees);
    println!("Part 2: {}", max_score);

}