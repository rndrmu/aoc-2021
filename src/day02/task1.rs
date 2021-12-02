use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Coordinates {
    horizontal: i32,
    depth: i32,
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.horizontal, self.depth)
    }
}


pub fn run() {
    let mut file = File::open("src/day02/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();


    //let mut coordinates: Vec<Coordinates> = Vec::new();
    let mut coords = Coordinates {
        horizontal: 0,
        depth: 0,
    };

    for line in contents.lines() {
        //println!("{}", line);

        let mut split = line.split(" ");
        let direction = split.nth(0).unwrap();
        let distance = split.nth(0).unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => {
                coords.horizontal += distance;
            },
            "down" => {
                coords.depth += distance;
            },
            "up" => {
                coords.depth -= distance;
            },
            _ => println!("unknown direction {}", direction),
        }

    }

    println!("{:?}", coords);

    // What do you get if you multiply your final horizontal position by your final depth?
    println!("{}", coords.horizontal * coords.depth);

}