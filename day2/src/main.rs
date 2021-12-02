use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    first_part();
    second_part()
}

fn second_part() {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    if let Ok(lines) = read_lines("./input/data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                let split = l.split(" ");
                let vec: Vec<&str> = split.collect();

                // println!("command: {} value: {}", vec[0], vec[1]);
                let value = vec[1].parse::<i32>().unwrap();

                match vec[0] {
                    "forward" => {
                        horizontal += value;
                        depth += aim * value;
                    },
                    "up" => aim -= value,
                    "down" => aim += value,
                    _ => panic!()
                }
            }
        }
    }
    println!("depth: {}, horizontal: {}, product: {} ", depth, horizontal, depth * horizontal)
}

fn first_part() {
    let mut depth = 0;
    let mut horizontal = 0;

    if let Ok(lines) = read_lines("./input/data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                let split = l.split(" ");
                let vec: Vec<&str> = split.collect();

                // println!("command: {} measure: {}", vec[0], vec[1]);

                match vec[0] {
                    "forward" => horizontal += vec[1].parse::<i32>().unwrap(),
                    "up" => depth -= vec[1].parse::<i32>().unwrap(),
                    "down" => depth += vec[1].parse::<i32>().unwrap(),
                    _ => panic!()
                }
            }
        }
    }

    println!("depth: {}, horizontal: {}, product: {} ", depth, horizontal, depth * horizontal)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
