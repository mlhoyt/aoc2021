use aoc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file("input/day11.txt")?;
    let data = parse(&input)?;
    println!("part1: {:?}", part1(&data));
    println!("part2: {:?}", part2(&data));

    Ok(())
}

type Grid2D = aoc::grid2d::Grid2D<usize>;
type Grid2DPoint = (usize, usize);

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
    let mut data = data.clone();
    let total_flashes = (0..100)
        .map(|_| {
            let (next_data, nr_flashes) = step(&data);
            data = next_data;

            nr_flashes
        })
        .sum();

    Ok(total_flashes)
}

fn part2(data: &Grid2D) -> Result<usize, Box<dyn std::error::Error>> {
    let mut data = data.clone();
    let expected_step_flashes = data.iter().count();
    let mut step_flashes = 0;
    let mut step_nr = 0;

    while step_flashes != expected_step_flashes {
        let (next_data, nr_flashes) = step(&data);
        data = next_data;

        step_flashes = nr_flashes;
        step_nr += 1;
    }

    Ok(step_nr)
}

fn step(data: &Grid2D) -> (Grid2D, usize) {
    // increment each point value by 1
    let mut data: Grid2D = data
        .iter()
        .map(|mut p| {
            p.value += 1;

            p
        })
        .collect();

    // determine the initial points to flash (value > 9)
    let mut new_flash_points = get_flashed_points(&data);

    // track the complete set of points flashed this step
    let mut curr_flashed_points = new_flash_points.clone();

    // while there are points to flash
    while new_flash_points.len() > 0 {
        // initialize a collection (map) of point changes for this iteration
        let mut point_changes = std::collections::HashMap::<Grid2DPoint, usize>::new();

        // populate the collection of point changes for this iteration
        new_flash_points.iter().for_each(|(y, x)| {
            get_adjacent_points(&data, (*y, *x))
                .iter()
                .for_each(|(y, x)| {
                    let point_change = point_changes.entry((*y, *x)).or_insert(0);
                    *point_change += 1;
                });
        });

        // update point values based on the point changes entries
        data = data
            .iter()
            .map(|mut p| {
                if let Some(v) = point_changes.get(&(p.y, p.x)) {
                    p.value += *v;
                }

                p
            })
            .collect();

        // get the new complete set of points flashed
        let next_flashed_points = get_flashed_points(&data);

        // update the points to flash in the next iteration
        // - The set difference is because each point can only flash once per step so we need the
        // set of points that have not yet flashed.
        new_flash_points = &next_flashed_points - &curr_flashed_points;

        // update the complete set of points flashed this step
        curr_flashed_points = next_flashed_points;
    }

    // reset flashed point values to zero
    data = data
        .iter()
        .map(|mut p| {
            if p.value > 9 {
                p.value = 0;
            }

            p
        })
        .collect();

    (data, curr_flashed_points.len())
}

fn get_flashed_points(data: &Grid2D) -> std::collections::HashSet<Grid2DPoint> {
    data.iter()
        .filter(|p| p.value > 9)
        .map(|p| (p.y, p.x))
        .collect()
}

fn get_adjacent_points(data: &Grid2D, point: Grid2DPoint) -> Vec<Grid2DPoint> {
    let y = point.0 as isize;
    let x = point.1 as isize;

    vec![
        (y - 1, x - 1),
        (y - 1, x),
        (y - 1, x + 1),
        (y, x - 1),
        (y, x),
        (y, x + 1),
        (y + 1, x - 1),
        (y + 1, x),
        (y + 1, x + 1),
    ]
    .iter()
    // keep valid (positive) coordinates
    .filter(|(y, x)| *y >= 0 && *x >= 0)
    // convert coordinates back to unsigned values
    .map(|(y, x)| (*y as usize, *x as usize))
    // keep points that are within the grid (which should be all of them)
    .filter_map(|(y, x)| match data.get_yx(y, x) {
        Some(_) => Some((y, x)),
        _ => None,
    })
    .collect()
}

#[test]
fn test_simulate_sample() {
    let input = aoc::read_file("input/day11.test.txt");
    assert!(input.is_ok());
    let input = input.unwrap();

    let data = parse(&input);
    assert!(data.is_ok());
    let data = data.unwrap();

    let result = part1(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1656);

    let result = part2(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 195);
}

#[test]
fn test_simulate_full() {
    let input = aoc::read_file("input/day11.txt");
    assert!(input.is_ok());
    let input = input.unwrap();

    let data = parse(&input);
    assert!(data.is_ok());
    let data = data.unwrap();

    let result = part1(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1793);

    let result = part2(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 247);
}
