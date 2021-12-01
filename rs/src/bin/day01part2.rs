use aoc;

// const FILE_NAME: &str = "input/day01.test.txt";
const FILE_NAME: &str = "input/day01.txt";

fn main() {
    let input = aoc::read_file(FILE_NAME).expect("cannot read file");

    let values: Vec<usize> = input.lines().filter_map(|v| v.parse().ok()).collect();

    let values: Vec<usize> = values.windows(3).map(|v| v.iter().sum()).collect();

    let result: usize = values
        .windows(2)
        .map(|vs| if vs[0] < vs[1] { 1 } else { 0 })
        .sum();

    println!("{}", result);
}
