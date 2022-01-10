use aoc;

// const FILE_NAME: &str = "input/day03.test.txt";
const FILE_NAME: &str = "input/day03.txt";

fn main() {
    let input = aoc::read_file(FILE_NAME).expect("cannot read file");

    // Parse the input into a list of lists of bools (not 1's and 0's)
    let codes = parser::parse(&input).expect("cannot parse input");
    // println!("{:#?}", codes);

    let code_len = codes[0].len();
    let codes_mid = (codes.len() / 2) as f32;

    // Gamma Rate is the most common value for the corresponding position found by summing the true
    // values and comparing the resulting sum against the middle "number of values".
    // NOTE: This is a SIMD approach to the "most common" (or majority) value per position.
    let gamma = codes
        .iter()
        .fold(vec![0; code_len], |acc, v| {
            acc.iter()
                .zip(v.iter())
                .map(|(sum, v)| sum + if *v { 1 } else { 0 })
                .collect::<Vec<_>>()
        })
        .into_iter()
        .map(|v| v as f32 > codes_mid)
        .collect::<Vec<_>>();

    let gamma_int = rate_to_usize(&gamma);
    println!("{:?} {}", gamma, gamma_int);

    // Gamma Rate is the least common value for the corresponding position found by inverting the
    // result of the Gamma Rate.
    let epsilon = gamma.iter().map(|&v| !v).collect::<Vec<_>>();

    let epsilon_int = rate_to_usize(&epsilon);
    println!("{:?} {}", epsilon, epsilon_int);

    let result = gamma_int * epsilon_int;
    println!("{}", result);
}

type Code = Vec<bool>;

fn rate_to_usize(rate: &[bool]) -> usize {
    usize::from_str_radix(
        rate.iter()
            .map(|&v| if v { "1" } else { "0" })
            .collect::<Vec<_>>()
            .join("")
            .as_str(),
        2,
    )
    .unwrap()
}

mod parser {
    use crate::Code;

    pub type ParseErr = peg::error::ParseError<peg::str::LineCol>;

    pub fn parse(s: &str) -> Result<Vec<Code>, ParseErr> {
        parser::parse(s)
    }

    peg::parser! {
        grammar parser() for str {
            pub rule parse() -> Vec<Code>
                = vs:code() ++ eol()
                {
                    vs
                }

            rule code() -> Code
                = ns:$(['0'..='1']+)
                {
                    ns.chars().map(|v| v == '1').collect::<Code>()
                }

            rule eol()
                = "\n"
                / "\r"
                / "\r" "\n"
        }
    }
}
