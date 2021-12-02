use aoc;

// const FILE_NAME: &str = "input/day02.test.txt";
const FILE_NAME: &str = "input/day02.txt";

fn main() {
    let input = aoc::read_file(FILE_NAME).expect("cannot read file");

    let instrs = parser::parse(&input).expect("cannot parse input");
    // println!("{:#?}", instrs);

    let pos = instrs
        .into_iter()
        .fold(Position(0, 0, 0), |p, i| p.adjust(i));
    // println!("{:#?}", pos);

    let result = pos.horizontal() * pos.depth();
    println!("{}", result);
}

#[derive(Debug)]
pub struct Instruction(isize, isize, isize);

#[derive(Debug)]
struct Position(isize, isize, isize);

impl Position {
    fn adjust(self, other: Instruction) -> Self {
        match other {
            Instruction(_, 0, n) => Self(self.0, self.1, self.2 + n),
            Instruction(_, n, 0) => Self(self.0 + n, self.1 + (self.2 * n), self.2),
            _ => self,
        }
    }

    fn horizontal(&self) -> isize {
        self.0
    }

    fn depth(&self) -> isize {
        self.1
    }

    fn aim(&self) -> isize {
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
