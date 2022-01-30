use aoc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file("input/day08.txt")?;
    let data = parse(&input)?;
    println!("part1: {:?}", part1(&data));
    println!("part2: {:?}", part2(&data));

    Ok(())
}

fn parse(data: &str) -> Result<Data, parser::ParseError> {
    parser::parse(&data)
}

mod parser {
    use crate::{Data, Pattern, Scenario, Segment};

    pub type ParseError = peg::error::ParseError<peg::str::LineCol>;

    pub fn parse(s: &str) -> Result<Data, ParseError> {
        parser::parse(s)
    }

    peg::parser! {
        grammar parser() for str {
            pub rule parse() -> Data
                = ss:scenario() ++ eol()
                {
                    ss
                }

            rule scenario() -> Scenario
                = sps:pattern() ++ _ _ "|" _ ovs:pattern() ++ _
                {
                    Scenario::new(sps, ovs)
                }

            rule pattern() -> Pattern
                = ss:segment()+
                {
                    ss.into_iter().collect::<Pattern>()
                }

            rule segment() -> Segment
                = "a" { Segment::A }
                / "b" { Segment::B }
                / "c" { Segment::C }
                / "d" { Segment::D }
                / "e" { Segment::E }
                / "f" { Segment::F }
                / "g" { Segment::G }

            rule _()
                = [' ']+

            rule eol()
                = "\n"
                / "\r"
                / "\r" "\n"
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

pub type Pattern = std::collections::HashSet<Segment>;

fn new_full_pattern() -> Pattern {
    vec![
        Segment::A,
        Segment::B,
        Segment::C,
        Segment::D,
        Segment::E,
        Segment::F,
        Segment::G,
    ]
    .into_iter()
    .collect::<Pattern>()
}

fn pattern_to_digit(p: &Pattern) -> usize {
    let pattern_0: Pattern = vec![
        Segment::A,
        Segment::B,
        Segment::C,
        Segment::E,
        Segment::F,
        Segment::G,
    ]
    .into_iter()
    .collect();

    let pattern_1: Pattern = vec![Segment::C, Segment::F].into_iter().collect();

    let pattern_2: Pattern = vec![Segment::A, Segment::C, Segment::D, Segment::E, Segment::G]
        .into_iter()
        .collect();

    let pattern_3: Pattern = vec![Segment::A, Segment::C, Segment::D, Segment::F, Segment::G]
        .into_iter()
        .collect();

    let pattern_4: Pattern = vec![Segment::B, Segment::C, Segment::D, Segment::F]
        .into_iter()
        .collect();

    let pattern_5: Pattern = vec![Segment::A, Segment::B, Segment::D, Segment::F, Segment::G]
        .into_iter()
        .collect();

    let pattern_6: Pattern = vec![
        Segment::A,
        Segment::B,
        Segment::D,
        Segment::E,
        Segment::F,
        Segment::G,
    ]
    .into_iter()
    .collect();

    let pattern_7: Pattern = vec![Segment::A, Segment::C, Segment::F]
        .into_iter()
        .collect();

    let pattern_8: Pattern = vec![
        Segment::A,
        Segment::B,
        Segment::C,
        Segment::D,
        Segment::E,
        Segment::F,
        Segment::G,
    ]
    .into_iter()
    .collect();

    let pattern_9: Pattern = vec![
        Segment::A,
        Segment::B,
        Segment::C,
        Segment::D,
        Segment::F,
        Segment::G,
    ]
    .into_iter()
    .collect();

    if *p == pattern_0 {
        0
    } else if *p == pattern_1 {
        1
    } else if *p == pattern_2 {
        2
    } else if *p == pattern_3 {
        3
    } else if *p == pattern_4 {
        4
    } else if *p == pattern_5 {
        5
    } else if *p == pattern_6 {
        6
    } else if *p == pattern_7 {
        7
    } else if *p == pattern_8 {
        8
    } else if *p == pattern_9 {
        9
    } else {
        0
    }
}

#[derive(Debug)]
pub struct Scenario {
    signal_patterns: Vec<Pattern>,
    output_values: Vec<Pattern>,
}

impl Scenario {
    pub fn new(signal_patterns: Vec<Pattern>, output_values: Vec<Pattern>) -> Self {
        Self {
            signal_patterns,
            output_values,
        }
    }

    fn decoder(&self) -> Box<dyn Fn(Segment) -> Segment> {
        let mut encoder: std::collections::HashMap<Segment, Pattern> = vec![
            (Segment::A, new_full_pattern()),
            (Segment::B, new_full_pattern()),
            (Segment::C, new_full_pattern()),
            (Segment::D, new_full_pattern()),
            (Segment::E, new_full_pattern()),
            (Segment::F, new_full_pattern()),
            (Segment::G, new_full_pattern()),
        ]
        .into_iter()
        .collect();

        // The 2, 3, and 5 patterns all use 5 segments.  A 3 pattern will have a difference of 2
        // segments from either a 2 pattern or a 5 pattern.  The 2 pattern and the 5 pattern will
        // have a difference of 4 segments.  We can use this to determine which signal pattern is
        // the 3 pattern and apply the corresponding deductions.
        //
        // We also keep the possible_three_patterns minus the 3 pattern itself and store it as the
        // possible_two_patterns.
        //
        // Once the initial deductions (1, 7, 4, 3) are applied to the encoder we can build a
        // superset of the segments needed by the 2 pattern (or the 5 pattern but we arbitrarily
        // choose the 2 pattern).  Then whichever of the values in possible_two_patterns is a
        // subset of the superset is the 2 pattern and we can apply it as the last deduction
        // against the encoder.

        // Store the possible_three_patterns as determined by all patterns with a length of 5.
        let possible_three_patterns: Vec<_> = self
            .signal_patterns
            .iter()
            .filter(|v| v.len() == 5)
            .collect();

        // Identify the 3 pattern.
        let three_pattern: Pattern;
        if possible_three_patterns[0]
            .symmetric_difference(possible_three_patterns[1])
            .count()
            == 4
        {
            three_pattern = possible_three_patterns[2].clone();
        } else if possible_three_patterns[0]
            .symmetric_difference(possible_three_patterns[2])
            .count()
            == 4
        {
            three_pattern = possible_three_patterns[1].clone();
        } else {
            three_pattern = possible_three_patterns[0].clone();
        }

        // Store the possible_two_patterns as determined by possible_three_patterns and the 3
        // pattern.
        let possible_two_patterns: Vec<_> = possible_three_patterns
            .clone()
            .into_iter()
            .filter(|&v| *v != three_pattern)
            .collect();

        // Apply the 1, 7, 4, 3 pattern deductions to the encoder.
        self.signal_patterns.iter().for_each(|v| {
            if v.len() == 2 {
                vec![Segment::C, Segment::F].iter().for_each(|s| {
                    let entry = encoder.get_mut(s).unwrap();
                    *entry = entry.intersection(v).copied().collect::<Pattern>();
                });
                vec![Segment::A, Segment::B, Segment::D, Segment::E, Segment::G]
                    .iter()
                    .for_each(|s| {
                        let entry = encoder.get_mut(s).unwrap();
                        *entry = entry.difference(v).copied().collect::<Pattern>();
                    });
            } else if v.len() == 3 {
                vec![Segment::A, Segment::C, Segment::F]
                    .iter()
                    .for_each(|s| {
                        let entry = encoder.get_mut(s).unwrap();
                        *entry = entry.intersection(v).copied().collect::<Pattern>();
                    });
                vec![Segment::B, Segment::D, Segment::E, Segment::G]
                    .iter()
                    .for_each(|s| {
                        let entry = encoder.get_mut(s).unwrap();
                        *entry = entry.difference(v).copied().collect::<Pattern>();
                    });
            } else if v.len() == 4 {
                vec![Segment::B, Segment::C, Segment::D, Segment::F]
                    .iter()
                    .for_each(|s| {
                        let entry = encoder.get_mut(s).unwrap();
                        *entry = entry.intersection(v).copied().collect::<Pattern>();
                    });
                vec![Segment::A, Segment::E, Segment::G]
                    .iter()
                    .for_each(|s| {
                        let entry = encoder.get_mut(s).unwrap();
                        *entry = entry.difference(v).copied().collect::<Pattern>();
                    });
            } else if v.len() == 5 && *v == three_pattern {
                vec![Segment::A, Segment::C, Segment::D, Segment::F, Segment::G]
                    .iter()
                    .for_each(|s| {
                        let entry = encoder.get_mut(s).unwrap();
                        *entry = entry.intersection(v).copied().collect::<Pattern>();
                    });
                vec![Segment::B, Segment::E].iter().for_each(|s| {
                    let entry = encoder.get_mut(s).unwrap();
                    *entry = entry.difference(v).copied().collect::<Pattern>();
                });
            }
        });

        // Identify the 2 pattern.
        let two_pattern_superset: Pattern =
            vec![Segment::A, Segment::C, Segment::D, Segment::E, Segment::G]
                .iter()
                .map(|v| encoder.get(v).unwrap())
                .flatten()
                .copied()
                .collect();

        let two_pattern: Pattern;
        if possible_two_patterns[0].is_subset(&two_pattern_superset) {
            two_pattern = possible_two_patterns[0].clone();
        } else {
            two_pattern = possible_two_patterns[1].clone();
        }

        // Apply the 2 pattern deduction.
        self.signal_patterns.iter().for_each(|v| {
            if v.len() == 5 && *v == two_pattern {
                vec![Segment::A, Segment::C, Segment::D, Segment::E, Segment::G]
                    .iter()
                    .for_each(|s| {
                        let entry = encoder.get_mut(s).unwrap();
                        *entry = entry.intersection(v).copied().collect::<Pattern>();
                    });
                vec![Segment::B, Segment::F].iter().for_each(|s| {
                    let entry = encoder.get_mut(s).unwrap();
                    *entry = entry.difference(v).copied().collect::<Pattern>();
                });
            }
        });

        // The encoder maps from a decoded segment to an encoded segement.  To decode we need the
        // reverse.
        let mut decoder = std::collections::HashMap::<Segment, Segment>::new();
        encoder.iter().for_each(|(&k_d, v)| {
            // There is only one segment in each pattern.
            let k_e: Segment = v.iter().take(1).copied().collect::<Vec<_>>()[0];
            decoder.insert(k_e, k_d);
        });

        // Generate (and return) a closure that uses the decoder to map from an encoded segment to
        // the decoded segment.
        Box::new(move |from: Segment| decoder.get(&from).unwrap().clone())
    }
}

pub type Data = Vec<Scenario>;

fn part1(data: &Data) -> Result<usize, Box<dyn std::error::Error>> {
    let count = data
        .iter()
        .map(|s| {
            s.output_values
                .iter()
                .map(|p| p.len())
                .filter(|&v| v == 2 || v == 3 || v == 4 || v == 7)
                .count()
        })
        .sum();

    Ok(count)
}

#[test]
fn test_part1_sample() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file("input/day08.test.txt")?;
    let data = parse(&input)?;
    let result = part1(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 26);

    Ok(())
}

#[test]
fn test_part1_full() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file("input/day08.txt")?;
    let data = parse(&input)?;
    let result = part1(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 449);

    Ok(())
}

fn part2(data: &Data) -> Result<usize, Box<dyn std::error::Error>> {
    let count = data
        .iter()
        .map(|s| {
            let decoder = s.decoder();

            s.output_values
                .iter()
                .map(|p| p.iter().map(|v| decoder(*v)).collect())
                .map(|p| pattern_to_digit(&p))
                .fold(0, |acc, v| (acc * 10) + v)
        })
        .sum();

    Ok(count)
}

#[test]
fn test_part2_sample() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file("input/day08.test.txt")?;
    let data = parse(&input)?;
    let result = part2(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 61229);

    Ok(())
}

#[test]
fn test_part2_full() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file("input/day08.txt")?;
    let data = parse(&input)?;
    let result = part2(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 968175);

    Ok(())
}
