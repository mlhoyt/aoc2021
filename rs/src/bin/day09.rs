use aoc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file("input/day09.txt")?;
    let data = parse(&input)?;
    println!("part1: {:?}", part1(&data));
    println!("part2: {:?}", part2(&data));

    Ok(())
}

type Grid2D = aoc::grid2d::Grid2D<usize>;
type Grid2DPoint = aoc::grid2d::Grid2DPoint<usize>;

fn parse(data: &str) -> Result<Grid2D, parser::ParseError> {
    parser::parse(&data)
}

mod parser {
    use crate::Grid2D;

    pub type ParseError = peg::error::ParseError<peg::str::LineCol>;

    pub fn parse(s: &str) -> Result<Grid2D, ParseError> {
        parser::parse(s)
    }

    peg::parser! {
        grammar parser() for str {
            pub rule parse() -> Grid2D
                = rs:row() ++ eol()
                {
                    Grid2D::new(&rs).unwrap()
                }

            rule row() -> Vec<usize>
                = vs:$['0'..='9']+
                {
                    vs.into_iter().map(|v| v.parse().unwrap()).collect()
                }

            rule eol()
                = "\n"
                / "\r"
                / "\r" "\n"
        }
    }
}

fn part1(data: &Grid2D) -> Result<usize, Box<dyn std::error::Error>> {
    let n = get_low_points(data).iter().map(|p| p.value + 1).sum();

    Ok(n)
}

fn part2(data: &Grid2D) -> Result<usize, Box<dyn std::error::Error>> {
    let mut ns: Vec<_> = get_low_points(data)
        .iter()
        .map(|p| get_basin(data, p).len())
        .collect();

    ns.sort();

    let n = ns.iter().rev().take(3).product();
    Ok(n)
}

fn get_low_points(data: &Grid2D) -> Vec<Grid2DPoint> {
    data.iter()
        .filter(|p| {
            get_adjacent_points(data, p)
                .iter()
                // Check if all adjacent point values are greater than the current point
                .all(|pp| pp.value > p.value)
        })
        .collect()
}

fn get_basin(data: &Grid2D, p: &Grid2DPoint) -> Vec<Grid2DPoint> {
    let mut contained_points = std::collections::HashSet::<Grid2DPoint>::new();
    let mut queued_points = std::collections::VecDeque::<Grid2DPoint>::new();

    queued_points.push_back(p.clone());

    while !queued_points.is_empty() {
        // Get next queued point
        let curr_point = queued_points.pop_front().unwrap();

        // Add point to contained_points
        contained_points.insert(curr_point.clone());

        // Get adjacent points
        get_adjacent_points(data, &curr_point)
            .into_iter()
            // Keep the non-boundary points (i.e. boundary points have value 9)
            .filter(|p| p.value < 9)
            // Add points to queued_points if not in contained_points
            .for_each(|p| {
                if !contained_points.contains(&p) {
                    queued_points.push_back(p);
                }
            });
    }

    contained_points.into_iter().collect()
}

fn get_adjacent_points(data: &Grid2D, p: &Grid2DPoint) -> Vec<Grid2DPoint> {
    let y = p.y as isize;
    let x = p.x as isize;

    // Calculate the possible four adjacent points row and column positions
    vec![(y - 1, x), (y, x - 1), (y, x + 1), (y + 1, x)]
        .into_iter()
        // Remove those positions who cannot convert to usize
        .filter(|(r, c)| *r >= 0 && *c >= 0)
        // Get the points by their row (y) and column (x) position
        .filter_map(|(r, c)| match data.get_yx(r as usize, c as usize) {
            Some(v) => Some(Grid2DPoint {
                x: c as usize,
                y: r as usize,
                value: v,
            }),
            _ => None,
        })
        .collect()
}

#[test]
fn test_simulate_sample() {
    let input = aoc::read_file("input/day09.test.txt");
    assert!(input.is_ok());
    let input = input.unwrap();

    let data = parse(&input);
    assert!(data.is_ok());
    let data = data.unwrap();

    let result = part1(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 15);

    let result = part2(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1134);
}

#[test]
fn test_simulate_full() {
    let input = aoc::read_file("input/day09.txt");
    assert!(input.is_ok());
    let input = input.unwrap();

    let data = parse(&input);
    assert!(data.is_ok());
    let data = data.unwrap();

    let result = part1(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 494);

    let result = part2(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1048128);
}
