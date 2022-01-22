use aoc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file("input/day07.txt")?;
    let data = parse(&input)?;
    println!("part1: {:?}", part1(data.clone()));
    println!("part2: {:?}", part2(data.clone()));

    Ok(())
}

// Data represents a distribution of horizontal position to count at that position
type Data = std::collections::HashMap<usize, usize>;

fn parse(data: &str) -> Result<Data, Box<dyn std::error::Error>> {
    let values: Vec<usize> = data
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    let mut dist = Data::new();
    values.into_iter().for_each(|v| {
        let dv = dist.entry(v).or_insert(0);
        *dv += 1;
    });

    Ok(dist)
}

fn part1(data: Data) -> Result<usize, Box<dyn std::error::Error>> {
    let optimal_alignment = align(&data, |v| v)?;

    Ok(optimal_alignment.1)
}

fn part2(data: Data) -> Result<usize, Box<dyn std::error::Error>> {
    let optimal_alignment = align(&data, |v| (v * (v + 1)) / 2)?;

    Ok(optimal_alignment.1)
}

fn align(data: &Data, fuel_cost: fn(usize) -> usize) -> Result<(usize, usize), Error> {
    let min = data.keys().min();
    if let None = min {
        return Err(Error::new("failed to yield a minimum position from data"));
    }

    let max = data.keys().max();
    if let None = max {
        return Err(Error::new("failed to yield a maximum position from data"));
    }

    let result = (*(min.unwrap())..=*(max.unwrap()))
        .into_iter()
        .map(|i| {
            let fuel: usize = data
                .iter()
                .map(|(p, c)| fuel_cost((*p as isize - i as isize).abs() as usize) * *c)
                .sum();

            (i, fuel)
        })
        .min_by_key(|v| v.1);
    if let None = result {
        return Err(Error::new("failed to yield an optimal position from data"));
    }

    Ok(result.unwrap())
}

#[derive(Debug)]
struct Error {
    details: String,
}

impl Error {
    fn new(msg: &str) -> Error {
        Error {
            details: msg.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}

#[test]
fn test_align() {
    let input = aoc::read_file("input/day07.test.txt");
    assert!(input.is_ok());
    let input = input.unwrap();

    let data = parse(&input);
    assert!(data.is_ok());
    let data = data.unwrap();

    let result = align(&data, |v| v);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result, (2, 37));

    let result = align(&data, |v| (v * (v + 1)) / 2);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result, (5, 168));
}

#[test]
fn test_simulate_sample() {
    let input = aoc::read_file("input/day07.test.txt");
    assert!(input.is_ok());
    let input = input.unwrap();

    let data = parse(&input);
    assert!(data.is_ok());
    let data = data.unwrap();
    // println!("{:#?}", data);

    let result = part1(data.clone());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 37);

    let result = part2(data.clone());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 168);
}

#[test]
fn test_simulate_full() {
    let input = aoc::read_file("input/day07.txt");
    assert!(input.is_ok());
    let input = input.unwrap();

    let data = parse(&input);
    assert!(data.is_ok());
    let data = data.unwrap();

    let result = part1(data.clone());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 349357);

    let result = part2(data.clone());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 96708205);
}
