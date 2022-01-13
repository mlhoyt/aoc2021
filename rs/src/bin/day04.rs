use aoc;

// const FILE_NAME: &str = "input/day04.test.txt";
const FILE_NAME: &str = "input/day04.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file(FILE_NAME)?;

    // Parse the input into the draws (a list of numbers) and a list of boards (which are structs
    // with a single "cells" field that is a list of lists of numbers).
    let (draws, boards) = parser::parse(&input)?;
    // println!("{:#?}", draws);
    // println!("{:#?}", boards);

    let part1_result = part1(&draws, boards.clone());
    println!("{:?}", part1_result);
    if let Some((draw, score)) = part1_result {
        println!("part1: {}", (draw * score));
    }

    let part2_result = part2(&draws, boards.clone());
    println!("{:?}", part2_result);
    if let Some((draw, score)) = part2_result {
        println!("part2: {}", (draw * score));
    }

    Ok(())
}

mod parser {
    use crate::Board;

    pub type ParseErr = peg::error::ParseError<peg::str::LineCol>;

    pub fn parse(s: &str) -> Result<(Vec<isize>, Vec<Board>), ParseErr> {
        parser::parse(s)
    }

    peg::parser! {
        grammar parser() for str {
            pub rule parse() -> (Vec<isize>, Vec<Board>)
                = ds:draws() eol() bs:boards()
                {
                    (ds, bs)
                }

            rule draws() -> Vec<isize>
                = ns:number() ++ "," eol()
                {
                    ns
                }

            rule boards() -> Vec<Board>
                = bs:board() ++ (eol() eol())
                {
                    bs
                }

            rule board() -> Board
                = rs:row() ++ eol()
                {
                    Board::init(rs)
                }

            rule row() -> Vec<isize>
                = ns:number() ++ _
                {
                    ns
                }

            rule number() -> isize
                = [' ']* ns:$(['0'..='9']+)
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

#[derive(Debug, Clone)]
pub struct Board {
    cells: Vec<Vec<isize>>,
}

impl Board {
    fn init(rs: Vec<Vec<isize>>) -> Board {
        Board { cells: rs }
    }

    fn mark_cells(&mut self, v: &isize) {
        for r in self.cells.iter_mut() {
            for cell in r.iter_mut() {
                if *cell == *v {
                    *cell = -1;
                }
            }
        }
    }

    fn has_bingo(&self) -> bool {
        if self.cells[0].iter().all(|v| *v < 0)
            || self.cells[1].iter().all(|v| *v < 0)
            || self.cells[2].iter().all(|v| *v < 0)
            || self.cells[3].iter().all(|v| *v < 0)
            || self.cells[4].iter().all(|v| *v < 0)
            || (self.cells[0][0] < 0
                && self.cells[1][0] < 0
                && self.cells[2][0] < 0
                && self.cells[3][0] < 0
                && self.cells[4][0] < 0)
            || (self.cells[0][1] < 0
                && self.cells[1][1] < 0
                && self.cells[2][1] < 0
                && self.cells[3][1] < 0
                && self.cells[4][1] < 0)
            || (self.cells[0][2] < 0
                && self.cells[1][2] < 0
                && self.cells[2][2] < 0
                && self.cells[3][2] < 0
                && self.cells[4][2] < 0)
            || (self.cells[0][3] < 0
                && self.cells[1][3] < 0
                && self.cells[2][3] < 0
                && self.cells[3][3] < 0
                && self.cells[4][3] < 0)
            || (self.cells[0][4] < 0
                && self.cells[1][4] < 0
                && self.cells[2][4] < 0
                && self.cells[3][4] < 0
                && self.cells[4][4] < 0)
        {
            return true;
        }

        false
    }

    fn get_score(&self) -> isize {
        let mut score = 0;

        for r in self.cells.iter() {
            for cell in r.iter() {
                if *cell > 0 {
                    score += *cell;
                }
            }
        }

        score
    }
}

fn part1(draws: &[isize], mut boards: Vec<Board>) -> Option<(isize, isize)> {
    for draw in draws {
        for board in boards.iter_mut() {
            board.mark_cells(draw);
            if board.has_bingo() {
                return Some((*draw, board.get_score()));
            }
        }
    }

    None
}

fn part2(draws: &[isize], mut boards: Vec<Board>) -> Option<(isize, isize)> {
    let mut board_status = std::collections::HashMap::<usize, (usize, isize)>::new();

    for (di, draw) in draws.iter().enumerate() {
        for (bi, board) in boards.iter_mut().enumerate() {
            board.mark_cells(draw);
            if board.has_bingo() {
                board_status.entry(bi).or_insert((di, board.get_score()));
            }
        }
    }

    let last_board = board_status
        .iter()
        .max_by(|(_, (di_a, _)), (_, (di_b, _))| di_a.cmp(di_b));

    if let Some((_, (di, score))) = last_board {
        Some((draws[*di], *score))
    } else {
        None
    }
}
