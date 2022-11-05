use aoc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file("input/day13.txt")?;
    let (points, folds) = parse(&input)?;
    println!("part1: {:?}", part1(&points, &folds));
    println!("part2: {:?}", part2(&points, &folds));

    Ok(())
}

pub type Points = std::collections::HashSet<(usize, usize)>;

fn display_points(points: &Points) {
    let (max_x, max_y) = points.iter().fold((0, 0), |(max_x, max_y), p| {
        (
            if p.0 > max_x { p.0 } else { max_x },
            if p.1 > max_y { p.1 } else { max_y },
        )
    });

    let mut grid = vec![vec!["."; max_x + 1]; max_y + 1];

    points.iter().for_each(|p| grid[p.1][p.0] = "#");

    grid.iter().for_each(|r| println!("{}", r.join("")));
}

#[derive(Debug)]
pub enum Axis {
    X,
    Y,
}

#[derive(Debug)]
pub struct Fold(Axis, usize);

pub type Folds = Vec<Fold>;

fn parse(data: &str) -> Result<(Points, Folds), parser::ParseError> {
    parser::parse(&data)
}

mod parser {
    use crate::{Axis, Fold, Folds, Points};

    pub type ParseError = peg::error::ParseError<peg::str::LineCol>;

    pub fn parse(s: &str) -> Result<(Points, Folds), ParseError> {
        parser::parse(s)
    }

    peg::parser! {
        grammar parser() for str {
            pub rule parse() -> (Points, Folds)
                = ps:points() eol() eol() fs:folds()
                {
                    (ps, fs)
                }

            rule points() -> Points
                = ps:point() ++ eol()
                {
                    ps.into_iter().collect::<Points>()
                }

            rule point() -> (usize, usize)
                = x:number() "," y:number()
                {
                    (x, y)
                }

            rule folds() -> Folds
                = fs:fold() ++ eol()
                {
                    fs
                }

            rule fold() -> Fold
                = "fold" _ "along" _ a:axis() "=" v:number()
                {
                    Fold(a, v)
                }

            rule axis() -> Axis
                = "x" { Axis::X }
                / "y" { Axis::Y }

            rule number() -> usize
                = ns:$(['0'..='9']+)
                {
                    ns.parse().unwrap()
                }

            rule _()
                = [' ']+

            rule eol()
                = "\n"
                / "\r"
                / "\r" "\n"
        }
    }
}

fn part1(points: &Points, folds: &Folds) -> Result<usize, Box<dyn std::error::Error>> {
    let folded_points = do_fold(points, &folds[0]);
    Ok(folded_points.iter().count())
}

fn part2(points: &Points, folds: &Folds) -> Result<usize, Box<dyn std::error::Error>> {
    let folded_points = folds.iter().fold(points.clone(), |acc, f| do_fold(&acc, f));
    display_points(&folded_points);
    Ok(0)
}

fn do_fold(points: &Points, fold: &Fold) -> Points {
    let mut folded_points = Points::new();

    points.iter().for_each(|p| match *fold {
        Fold(Axis::X, value) => {
            if p.0 > value {
                folded_points.insert((value - (p.0 - value), p.1));
            } else {
                folded_points.insert((p.0, p.1));
            }
        }
        Fold(Axis::Y, value) => {
            if p.1 > value {
                folded_points.insert((p.0, (value - (p.1 - value))));
            } else {
                folded_points.insert((p.0, p.1));
            }
        }
    });

    folded_points
}

#[test]
fn test_simulate_sample() {
    let input = aoc::read_file("input/day13.test.txt");
    assert!(input.is_ok());
    let input = input.unwrap();

    let data = parse(&input);
    assert!(data.is_ok());
    let (points, folds) = data.unwrap();

    let result = part1(&points, &folds);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 17);

    let result = part2(&points, &folds);
    assert!(result.is_ok());
    // assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_simulate_full() {
    let input = aoc::read_file("input/day13.txt");
    assert!(input.is_ok());
    let input = input.unwrap();

    let data = parse(&input);
    assert!(data.is_ok());
    let (points, folds) = data.unwrap();

    let result = part1(&points, &folds);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 807);

    let result = part2(&points, &folds);
    assert!(result.is_ok());
    // assert_eq!(result.unwrap(), 0);
    // LGHEGUEJ
}
