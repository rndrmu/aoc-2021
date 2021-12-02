use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Coordinates {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.horizontal, self.depth, self.aim)
    }
}


pub fn run() {
    let mut file = File::open("src/day02/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut coords = Coordinates {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for line in contents.lines() {
        let mut split = line.split(" ");
        let direction = split.nth(0).unwrap();
        let distance = split.nth(0).unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => {
                if coords.aim == 0 {
                    coords.horizontal += distance;
                } else {
                    coords.horizontal += distance;
                    coords.depth += distance * coords.aim;
                }
            },
            "up" => {
                coords.aim -= distance;
            },
            "down" => {
                coords.aim += distance;
            },
            _ => println!("ayo"),
        }
    }

    println!("{:?}", coords);
    println!("{}", coords.horizontal * coords.depth);

}