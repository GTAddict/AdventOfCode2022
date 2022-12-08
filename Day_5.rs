use std::fs;

fn main() {

    let file_contents = fs::read_to_string("Input_5.txt").expect("File read error!");
    
    let mut split = file_contents.split(":");
    let (crates, instructions) = (split.next().unwrap(), split.next().unwrap());

    let num_stacks = crates.lines().nth_back(0).unwrap().len();
    
    // Outer vector is column, inner is row from bottom to top
    let mut stacks_part_1 : Vec<Vec<char>> = Vec::new();
    let mut stacks_part_2 : Vec<Vec<char>> = Vec::new();

    for i in 0..num_stacks
    {
        stacks_part_1.push(Vec::new());
        stacks_part_2.push(Vec::new());
        for line in crates.lines().rev()
        {
            let character = line.chars().nth(i).unwrap();
            if character != ' '
            {
                stacks_part_1[i].push(character);
                stacks_part_2[i].push(character);
            }
        }
    }

    for instruction in instructions.lines()
    {
        let mut split = instruction.split(";");
        let (num, source_and_dest) : (u32, &str) = (split.next().unwrap().parse::<u32>().unwrap(), split.next().unwrap());
        split = source_and_dest.split("-");
        let (source, dest) = (split.next().unwrap().parse::<u32>().unwrap() - 1, split.next().unwrap().parse::<u32>().unwrap() - 1);

        for _ in 0..num
        {
            let temp = stacks_part_1[source as usize].pop().unwrap();
            stacks_part_1[dest as usize].push(temp);
        }

        let range = stacks_part_2[source as usize].len() - num as usize..;
        let temp : Vec<char> = stacks_part_2[source as usize].drain(range).collect();
        stacks_part_2[dest as usize].extend_from_slice(&temp);
    }


    print!("Part 1: ");
    for stack in stacks_part_1
    {
        print!("{}", stack.last().unwrap());
    }
    println!("");

    print!("Part 2: ");
    for stack in stacks_part_2
    {
        print!("{}", stack.last().unwrap());
    }
    println!("");
}