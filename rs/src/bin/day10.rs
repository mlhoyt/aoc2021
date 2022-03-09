use aoc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file("input/day10.txt")?;
    let data = parse(&input);
    println!("part1: {:?}", part1(&data));
    println!("part2: {:?}", part2(&data));

    Ok(())
}

fn parse(data: &str) -> Vec<TunnelMap> {
    data.lines()
        .map(|l| {
            // state is a stack of the closing characters that correspond to each unmatched opening
            // character encountered so far
            let mut state: Vec<char> = Vec::new();

            for c in l.chars() {
                match c {
                    '[' | '(' | '{' | '<' => state.push(map_open_char_to_close_char(c).unwrap()),
                    ']' | ')' | '}' | '>' => match state.pop() {
                        None => return TunnelMap::ParseError,
                        Some(cc) => {
                            if cc != c {
                                return TunnelMap::Corrupt(cc, c);
                            }
                        }
                    },
                    _ => return TunnelMap::ParseError,
                }
            }

            match state.pop() {
                None => return TunnelMap::Complete,
                Some(cc) => {
                    state.push(cc);
                    return TunnelMap::Incomplete(state.into_iter().rev().collect());
                }
            }
        })
        .collect()
}

#[test]
fn test_parse() {
    let p1 = parse("[]");
    let e1 = TunnelMap::Complete;
    assert_eq!(p1[0], e1);

    let p2 = parse("[");
    let e2 = TunnelMap::Incomplete("]".to_string());
    assert_eq!(p2[0], e2);

    let p3 = parse("[}");
    let e3 = TunnelMap::Corrupt(']', '}');
    assert_eq!(p3[0], e3);

    let p4 = parse("[][]");
    let e4 = TunnelMap::Complete;
    assert_eq!(p4[0], e4);

    let p5 = parse("[][");
    let e5 = TunnelMap::Incomplete("]".to_string());
    assert_eq!(p5[0], e5);

    let p6 = parse("[][}");
    let e6 = TunnelMap::Corrupt(']', '}');
    assert_eq!(p6[0], e6);

    let p7 = parse("[({");
    let e7 = TunnelMap::Incomplete("})]".to_string());
    assert_eq!(p7[0], e7);

    let p10 = parse("[({(<(())[]>[[{[]{<()<>>");
    let e10 = TunnelMap::Incomplete("}}]])})]".to_string());
    assert_eq!(p10[0], e10);
}

#[derive(Debug, Clone, PartialEq)]
pub enum TunnelMap {
    Complete,
    Incomplete(String),
    Corrupt(char, char),
    ParseError,
}

fn map_open_char_to_close_char(c: char) -> Option<char> {
    match c {
        '[' => Some(']'),
        '(' => Some(')'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

fn map_close_char_to_error_points(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn map_close_char_to_completion_points(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn part1(data: &[TunnelMap]) -> Result<usize, Box<dyn std::error::Error>> {
    let rv = data
        .iter()
        .map(|v| match v {
            TunnelMap::Corrupt(_, a) => map_close_char_to_error_points(*a),
            _ => 0,
        })
        .sum();

    Ok(rv)
}

fn part2(data: &[TunnelMap]) -> Result<usize, Box<dyn std::error::Error>> {
    let mut rv: Vec<_> = data
        .iter()
        .filter_map(|v| match v {
            TunnelMap::Incomplete(s) => Some(
                s.chars()
                    .map(|c| map_close_char_to_completion_points(c))
                    .fold(0, |acc, v| acc * 5 + v),
            ),
            _ => None,
        })
        .collect();

    rv.sort();
    // We were told to assume there is always an odd number so a simple "middle value" is the
    // median.
    let mi = rv.len() / 2;
    Ok(rv[mi])
}

#[test]
fn test_simulate_sample() {
    let input = aoc::read_file("input/day10.test.txt");
    assert!(input.is_ok());
    let input = input.unwrap();

    let data = parse(&input);
    // println!("[simulate_sample:parse] data:{:#?}", data);

    let result = part1(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 26397);

    let result = part2(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 288957);
}

#[test]
fn test_simulate_full() {
    let input = aoc::read_file("input/day10.txt");
    assert!(input.is_ok());
    let input = input.unwrap();

    let data = parse(&input);

    let result = part1(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 168417);

    let result = part2(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2802519786);
}
