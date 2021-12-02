use aoc;
use std::ops::Add;

// const FILE_NAME: &str = "input/day02.test.txt";
const FILE_NAME: &str = "input/day02.txt";

fn main() {
    let input = aoc::read_file(FILE_NAME).expect("cannot read file");

    let instrs = parser::parse(&input).expect("cannot parse input");
    // println!("{:#?}", instrs);

    let pos = instrs
        .into_iter()
        .fold(Instruction(0, 0, 0), |acc, v| acc + v);
    // println!("{:#?}", pos);

    let result = pos.y() * pos.z();
    println!("{}", result);
}

#[derive(Debug)]
pub struct Instruction(isize, isize, isize);

impl Add for Instruction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Instruction {
    fn x(&self) -> isize {
        self.0
    }

    fn y(&self) -> isize {
        self.1
    }

    fn z(&self) -> isize {
        self.2
    }
}

mod parser {
    use crate::Instruction;

    pub type ParseErr = peg::error::ParseError<peg::str::LineCol>;

    pub fn parse(s: &str) -> Result<Vec<Instruction>, ParseErr> {
        parser::parse(s)
    }

    peg::parser! {
        grammar parser() for str {
            pub rule parse() -> Vec<Instruction>
                = ns:instruction() ++ eol()
                {
                    ns
                }

            rule instruction() -> Instruction
                = "forward" _ n:number() {Instruction(0, n, 0)}
                / "up" _ n:number() {Instruction(0, 0, (-1 * n))}
                / "down" _ n:number() {Instruction(0, 0, n)}

            rule number() -> isize
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
