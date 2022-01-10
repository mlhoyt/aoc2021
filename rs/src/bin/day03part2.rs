use aoc;

// const FILE_NAME: &str = "input/day03.test.txt";
const FILE_NAME: &str = "input/day03.txt";

fn main() {
    let input = aoc::read_file(FILE_NAME).expect("cannot read file");

    // Parse the input into a list of lists of bools (not 1's and 0's)
    let codes = parser::parse(&input).expect("cannot parse input");
    // println!("{:#?}", codes);

    // Oxygen Generator Rate reduces the list of codes based on the most common value in a position
    // (defaulting to 1's if the 1's and 0's counts are equal) followed by the codes that match
    // that value in the subsequent position until all positions are processed and only one value
    // remains.
    // NOTE: This is a SISD approach to the "most common" (or majority) value per position.
    let mut oxygen_codes = codes.clone();
    (0..codes[0].len()).for_each(|i| {
        if oxygen_codes.len() <= 1 {
            return;
        }

        let v = calc_codes_majority_value_for_pos(&oxygen_codes, i);

        oxygen_codes = oxygen_codes
            .clone()
            .into_iter()
            .filter(|code| code[i] == v)
            .collect::<Vec<_>>();
    });

    let oxygen_int = rate_to_usize(&oxygen_codes[0]);
    println!("{:?} {}", oxygen_codes[0], oxygen_int);

    // CO2 Scrubber Rate is basically the inverse of the Oxygen Generator Rate using the least
    // common value per position (and defaulting to 0 if counts are equal).
    let mut co2_scrubber_codes = codes.clone();
    (0..codes[0].len()).for_each(|i| {
        if co2_scrubber_codes.len() <= 1 {
            return;
        }

        let v = !calc_codes_majority_value_for_pos(&co2_scrubber_codes, i);

        co2_scrubber_codes = co2_scrubber_codes
            .clone()
            .into_iter()
            .filter(|code| code[i] == v)
            .collect::<Vec<_>>();
    });

    let co2_scrubber_int = rate_to_usize(&co2_scrubber_codes[0]);
    println!("{:?} {}", co2_scrubber_codes[0], co2_scrubber_int);

    let result = oxygen_int * co2_scrubber_int;
    println!("{}", result);
}

type Code = Vec<bool>;

fn calc_codes_majority_value_for_pos(codes: &[Code], pos: usize) -> bool {
    let codes_mid = codes.len() as f32 / 2 as f32;
    let count: usize = codes.iter().map(|code| if code[pos] { 1 } else { 0 }).sum();

    count as f32 >= codes_mid
}

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
