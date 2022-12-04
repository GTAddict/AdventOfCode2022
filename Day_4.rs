use std::fs;
use std::collections::HashSet;

fn main() {

    let file_contents = fs::read_to_string("Input_4.txt").expect("File read error!");
    let mut fully_contained_pairs : u32 = 0;
    let mut intersecting_pairs : u32 = 0;
    
    for line in file_contents.lines()
    {
        let pair : Vec<&str> = line.split(",").collect();
        let (mut set_a, mut set_b) = (HashSet::new(), HashSet::new());
        let (mut range_begin, mut range_end) : (u32, u32) = (pair[0].split("-").collect::<Vec<&str>>()[0].parse().unwrap(), pair[0].split("-").collect::<Vec<&str>>()[1].parse().unwrap());
        for i in range_begin..=range_end
        {
            set_a.insert(i);
        }

        (range_begin, range_end) = (pair[1].split("-").collect::<Vec<&str>>()[0].parse().unwrap(), pair[1].split("-").collect::<Vec<&str>>()[1].parse().unwrap());
        for i in range_begin..=range_end
        {
            set_b.insert(i);
        }

        if set_a.is_subset(&set_b) || set_b.is_subset(&set_a)
        {
            fully_contained_pairs += 1;
        }

        if set_a.intersection(&set_b).copied().collect::<HashSet<u32>>().len() > 0
        {
            intersecting_pairs += 1;
        }
    }

    println!("Part 1: {}", fully_contained_pairs);
    println!("Part 2: {}", intersecting_pairs);

}