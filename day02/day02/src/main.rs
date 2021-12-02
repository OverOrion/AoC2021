use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use std::str::FromStr;
use std::num::ParseIntError;


struct Command {
    kind: String,
    unit: i32,
}

struct Position {
    depth: i32,
    horizontal_position: i32,
    aim: i32, 
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split_whitespace()
            .collect();

        let kind_fromstr = coords[0];
        let unit_fromstr = coords[1].parse::<i32>()?;

        Ok( Command{ kind: kind_fromstr.to_string(), unit: unit_fromstr })

    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<Command> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|line| line.unwrap().parse::<Command>().unwrap())
        .collect()
}

fn movement_part1(depth: &mut i32, horizontal_position: &mut i32, _aim: &mut i32, command: &Command){
    match command.kind.as_str() {
        "forward" => {
            *horizontal_position += command.unit;
        },
        "down" => {
            *depth += command.unit;
        },
        "up" => {
            *depth -= command.unit;
        }
        _ =>{
            panic!("Missing movement kind");
        }
    }
}


fn execute_moves(depth: &mut i32, horizontal_position: &mut i32, aim: &mut i32, commands: &Vec<Command>, movement: fn(&mut i32, &mut i32, &mut i32, &Command)){
    for cmd in commands{
        movement(&mut *depth, &mut *horizontal_position, &mut *aim, &cmd);
    }
}



fn movement_part2(depth: &mut i32, horizontal_position: &mut i32, aim: &mut i32, command: &Command){
    match command.kind.as_str() {
        "forward" => {
            *horizontal_position += command.unit;
            *depth += *aim * command.unit;
        },
        "down" => {
            *aim += command.unit;
        },
        "up" => {
            *aim -= command.unit;
        }
        _ =>{
            panic!("Missing movement kind");
        }
    }

}

fn main() {
    let data = lines_from_file("day_02.txt");

    let mut p = Position {depth: 0, horizontal_position: 0, aim: 0};

    // Part 1
    execute_moves(&mut p.depth, &mut p.horizontal_position, &mut p.aim, &data, movement_part1);
    println!("Part 1: depth: {}, horizontal_position: {}", p.depth, p.horizontal_position);
    println!("Part 1: solution: {}", p.depth * p.horizontal_position);


    // Part 2
    let mut p = Position {depth: 0, horizontal_position: 0, aim: 0};

    execute_moves(&mut p.depth, &mut p.horizontal_position, &mut p.aim, &data, movement_part2);
    println!("Part 2: depth: {}, horizontal_position: {}, aim: {}", p.depth, p.horizontal_position, p.aim);
    println!("Part 2: solution: {}", p.horizontal_position * p.depth);


}
