use std::fs::File;
use std::io::prelude::*;

pub fn run() {
    let mut file = File::open("src/day01/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("{:?}", contents.lines());
    let depths = contents.lines().map(|line| line.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut nig = Vec::<i32>::new();

    for (pos, line) in contents.lines().enumerate() {
        if pos == 0 {
        print!("{} (N/A - no previous measurement)\n", line);
        } else if depths[pos] > depths[pos - 1] {
            nig.push(depths[pos]);
            println!("{} (Increased)", depths[pos]);
            } else {
                println!("{} (Decreased)", depths[pos]);
            }
    }

    println!("\nTotal depths increases: {:?}", nig.len());

    // let prev_depths = contents.lines().map(|x|x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    // let mut deeper_than_prev = Vec::<i32>::new();

    // for (pos, i) in prev_depths.iter().enumerate() {
    //     let mut next_depths = prev_depths.clone();
        
    //     if pos == 0 {
    //         print!("{} (N/A - no previous measurement)", i);
    //     } else if next_depths[pos] > next_depths[pos - 1] {
    //         deeper_than_prev.push(next_depths[pos]);
    //         println!("{} (Increased)", next_depths[pos]);
    //     } else {
    //         println!("{} (Decreased)", next_depths[pos]);
    //     }
    // }
}