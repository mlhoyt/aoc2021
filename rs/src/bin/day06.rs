use aoc;

// const FILE_NAME: &str = "input/day06.test.txt";
const FILE_NAME: &str = "input/day06.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file(FILE_NAME)?;

    // Parse the input into a list of unsigned integers
    let fish = parse(&input)?;
    // println!("{:?}", fish);

    let part1_result = simulate(fish.clone(), 80);
    println!("part1: {:?}", part1_result);

    let part2_result = simulate(fish.clone(), 256);
    println!("part2: {:?}", part2_result);

    Ok(())
}

fn parse(input: &str) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let values: Vec<usize> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    let mut dist = vec![0; 9];
    values.into_iter().for_each(|v| dist[v] += 1);

    Ok(dist)
}

fn simulate(mut dist: Vec<usize>, nr_days: usize) -> usize {
    for _ in 0..nr_days {
        let nr_spawning = dist[0];
        (0..8).into_iter().for_each(|i| dist[i] = dist[i + 1]);
        dist[6] += nr_spawning;
        dist[8] = nr_spawning;
    }

    dist.into_iter().sum()
}

#[test]
fn test_simulate_sample() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file("input/day06.test.txt")?;
    let fish = parse(&input)?;
    assert_eq!(simulate(fish.clone(), 1), 5);
    assert_eq!(simulate(fish.clone(), 2), 6);
    assert_eq!(simulate(fish.clone(), 3), 7);
    assert_eq!(simulate(fish.clone(), 4), 9);
    assert_eq!(simulate(fish.clone(), 5), 10);
    assert_eq!(simulate(fish.clone(), 80), 5934);
    assert_eq!(simulate(fish.clone(), 256), 26984457539);

    Ok(())
}

#[test]
fn test_simulate_full() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file("input/day06.txt")?;
    let fish = parse(&input)?;
    assert_eq!(simulate(fish.clone(), 80), 371379);
    assert_eq!(simulate(fish.clone(), 256), 1674303997472);

    Ok(())
}
