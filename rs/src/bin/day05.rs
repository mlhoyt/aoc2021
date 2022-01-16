use aoc;

// const FILE_NAME: &str = "input/day05.test.txt";
const FILE_NAME: &str = "input/day05.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file(FILE_NAME)?;

    // Parse the input into a list of Line instances where each contains two Points consisting of
    // an x and a y coordinate.
    let lines = parser::parse(&input)?;
    // println!("lines:\n{:#?}", lines);

    let part1_result = part1(&lines);
    println!("part1: {:?}", part1_result);

    let part2_result = part2(&lines);
    println!("part2: {:?}", part2_result);

    Ok(())
}

mod parser {
    use crate::{Line, Point};

    pub type ParseErr = peg::error::ParseError<peg::str::LineCol>;

    pub fn parse(s: &str) -> Result<Vec<Line>, ParseErr> {
        parser::parse(s)
    }

    peg::parser! {
        grammar parser() for str {
            pub rule parse() -> (Vec<Line>)
                = ls:line() ++ eol()
                {
                    ls
                }

            rule line() -> Line
                = x1:number() "," y1:number() _ "->" _ x2:number() "," y2:number()
                {
                    Line(
                        Point::new(x1, y1),
                        Point::new(x2, y2),
                    )
                }

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

#[derive(Debug)]
pub struct Line(Point, Point);

impl Line {
    fn is_vertical(&self) -> bool {
        self.0.x == self.1.x
    }

    fn is_horizontal(&self) -> bool {
        self.0.y == self.1.y
    }
}

#[test]
fn test_line_is_vertical() {
    struct TestCase {
        data: Line,
        expected: bool,
    }
    let tests = vec![
        TestCase {
            data: Line(Point::new(1, 1), Point::new(1, 3)),
            expected: true,
        },
        TestCase {
            data: Line(Point::new(1, 1), Point::new(3, 1)),
            expected: false,
        },
        TestCase {
            data: Line(Point::new(1, 1), Point::new(3, 3)),
            expected: false,
        },
    ];

    tests
        .iter()
        .for_each(|t| assert_eq!((t.data).is_vertical(), t.expected));
}

#[test]
fn test_line_is_horizontal() {
    struct TestCase {
        data: Line,
        expected: bool,
    }
    let tests = vec![
        TestCase {
            data: Line(Point::new(1, 1), Point::new(1, 3)),
            expected: false,
        },
        TestCase {
            data: Line(Point::new(1, 1), Point::new(3, 1)),
            expected: true,
        },
        TestCase {
            data: Line(Point::new(1, 1), Point::new(3, 3)),
            expected: false,
        },
    ];

    tests
        .iter()
        .for_each(|t| assert_eq!((t.data).is_horizontal(), t.expected));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn to(&self, other: &Self) -> Vec<Self> {
        let delta_y = other.y as isize - self.y as isize;
        let delta_x = other.x as isize - self.x as isize;

        let (step_y, step_x) = if delta_y.abs() == delta_x.abs() {
            (delta_y.signum(), delta_x.signum())
        } else if delta_y == 0 {
            (0, delta_x.signum())
        } else if delta_x == 0 {
            (delta_y.signum(), 0)
        } else if delta_y.abs() > delta_x.abs() {
            unimplemented!()
        } else {
            unimplemented!()
        };

        let (mut curr_x, mut curr_y) = (self.x, self.y);
        let mut points = vec![Point {
            x: curr_x,
            y: curr_y,
        }];
        while (curr_x, curr_y) != (other.x, other.y) {
            let next_x = (curr_x as isize + step_x) as usize;
            let next_y = (curr_y as isize + step_y) as usize;

            points.push(Self {
                x: next_x,
                y: next_y,
            });

            curr_x = next_x;
            curr_y = next_y;
        }

        points
    }
}

#[test]
fn test_point_to() {
    struct TestCase {
        point: Point,
        other: Point,
        expected: Vec<Point>,
    }
    let tests = vec![
        TestCase {
            // vertical+N
            point: Point::new(1, 1),
            other: Point::new(1, 3),
            expected: vec![Point::new(1, 1), Point::new(1, 2), Point::new(1, 3)],
        },
        TestCase {
            // horizontal+E
            point: Point::new(1, 1),
            other: Point::new(3, 1),
            expected: vec![Point::new(1, 1), Point::new(2, 1), Point::new(3, 1)],
        },
        TestCase {
            // vertical+S
            point: Point::new(1, 3),
            other: Point::new(1, 1),
            expected: vec![Point::new(1, 3), Point::new(1, 2), Point::new(1, 1)],
        },
        TestCase {
            // horizontal+W
            point: Point::new(3, 1),
            other: Point::new(1, 1),
            expected: vec![Point::new(3, 1), Point::new(2, 1), Point::new(1, 1)],
        },
        TestCase {
            // diagonal+NE
            point: Point::new(1, 1),
            other: Point::new(3, 3),
            expected: vec![Point::new(1, 1), Point::new(2, 2), Point::new(3, 3)],
        },
        TestCase {
            // diagnoal+SE
            point: Point::new(1, 3),
            other: Point::new(3, 1),
            expected: vec![Point::new(1, 3), Point::new(2, 2), Point::new(3, 1)],
        },
        TestCase {
            // diagonal+SW
            point: Point::new(3, 3),
            other: Point::new(1, 1),
            expected: vec![Point::new(3, 3), Point::new(2, 2), Point::new(1, 1)],
        },
        TestCase {
            // diagnoal+NW
            point: Point::new(3, 1),
            other: Point::new(1, 3),
            expected: vec![Point::new(3, 1), Point::new(2, 2), Point::new(1, 3)],
        },
    ];

    tests
        .iter()
        .for_each(|t| assert_eq!(&(t.point).to(&t.other), &t.expected));
}

fn part1(lines: &[Line]) -> Option<usize> {
    let mut map = std::collections::HashMap::<Point, usize>::new();

    lines
        .iter()
        .filter(|l| l.is_vertical() || l.is_horizontal())
        .map(|l| (l.0).to(&l.1))
        .flatten()
        .for_each(|pt| {
            let entry = map.entry(pt).or_insert(0);
            *entry += 1;
        });

    Some(map.into_iter().filter(|(_, count)| *count > 1).count())
}

#[test]
fn test_part1_sample() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file("input/day05.test.txt")?;
    let lines = parser::parse(&input)?;
    assert_eq!(part1(&lines), Some(5));

    Ok(())
}

#[test]
fn test_part1_full() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file("input/day05.txt")?;
    let lines = parser::parse(&input)?;
    assert_eq!(part1(&lines), Some(7269));

    Ok(())
}

fn part2(lines: &[Line]) -> Option<usize> {
    let mut map = std::collections::HashMap::<Point, usize>::new();

    lines
        .iter()
        // do not filter any lines
        .map(|l| (l.0).to(&l.1))
        .flatten()
        .for_each(|pt| {
            let entry = map.entry(pt).or_insert(0);
            *entry += 1;
        });

    Some(map.into_iter().filter(|(_, count)| *count > 1).count())
}

#[test]
fn test_part2_sample() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file("input/day05.test.txt")?;
    let lines = parser::parse(&input)?;
    assert_eq!(part2(&lines), Some(12));

    Ok(())
}

#[test]
fn test_part2_full() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file("input/day05.txt")?;
    let lines = parser::parse(&input)?;
    assert_eq!(part2(&lines), Some(21140));

    Ok(())
}
