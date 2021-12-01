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

    let mut i = 0;
    let mut j = 1;
    let mut k = 2;


    while k < numbers.len() {
        let sum = numbers[i] + numbers[j] + numbers[k];

        res.push(sum);

        i += 1;
        j += 1;
        k += 1;
    }

    res

}

fn count_increases(numbers: &Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = 1;

    let mut num_of_increases = 0;

    while i < numbers.len() - 1 {
        if numbers[i] < numbers[j] {
            num_of_increases += 1;
        }


        i += 1;
        j += 1;

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
