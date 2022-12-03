use std::fs;
use std::collections::HashSet;

fn main() {

    let file_contents = fs::read_to_string("Input_3.txt").expect("File read error!");
    
    let mut priority_sum : u32 = 0;
    
    for line in file_contents.lines()
    {
        let mut map = HashSet::new();
        let count = line.chars().count();
        for i in 0..count/2
        {
            map.insert(line.chars().nth(i));
        }

        for i in count/2..count
        {
            if map.get(&line.chars().nth(i)) != None
            {
                let mut priority = line.chars().nth(i).unwrap() as u32;
                if priority >= 97 && priority <= 122
                {
                    priority -= 96;
                }
                else if priority >= 65 && priority <= 90
                {
                    priority -= 38;
                }

                priority_sum += priority;
                break;
            }
        }
    }

    println!("Part 1: {}", priority_sum);

    let mut group_iter = 0;
    let mut common_priority_sum = 0;
    let mut common_set = HashSet::new();
    for line in file_contents.lines()
    {
        let mut set = HashSet::new();
        let count = line.chars().count();
        for i in 0..count
        {
            set.insert(line.chars().nth(i));
        }

        if group_iter == 0
        {
            common_set = set;
        }
        else if group_iter == 1
        {
            common_set = common_set.intersection(&set).copied().collect();
        }
        else if group_iter == 2
        {
            common_set = common_set.intersection(&set).copied().collect();

            for priority in common_set.iter()
            {
                let mut priority_int = priority.unwrap() as u32;
                if priority_int >= 97 && priority_int <= 122
                {
                    priority_int -= 96;
                }
                else if priority_int >= 65 && priority_int <= 90
                {
                    priority_int -= 38;
                }
    
                common_priority_sum += priority_int;
                break;
            }
        }
        else
        {
            println!("Wtf...");
        }

        
        group_iter = (group_iter + 1) % 3;
    }

    println!("Part 2: {}", common_priority_sum);
}