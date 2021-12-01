use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect()
}


fn sum_3(numbers: &Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();

    let mut it = numbers.windows(3);

    while let Some([i, j, k]) = it.next() {
        res.push(i+j+k);
    }

    res
}

fn count_increases(numbers: &Vec<i32>) -> i32 {
    let mut it = numbers.windows(2);
    let mut num_of_increases = 0;

    while let Some([left, right]) = it.next() {
        if left < right {
            num_of_increases += 1;
        }

    }

    num_of_increases
}


fn main() {

    let data = lines_from_file("day_01.txt");

    // Part 1
    let res = count_increases(&data);
    println!("Part 1: {}", res); 


    // Part 2
    let sums = sum_3(&data);
    let res = count_increases(&sums);

    println!("Part 2: {}", res); 
}
