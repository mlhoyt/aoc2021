use aoc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc::read_file("input/day12.txt")?;
    let data = parse(&input)?;
    println!("part1: {:?}", part1(&data));
    println!("part2: {:?}", part2(&data));

    Ok(())
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Node {
    Start,
    End,
    Big(String),
    Small(String),
}

pub type Tree = std::collections::HashMap<Node, std::collections::HashSet<Node>>;

fn parse(data: &str) -> Result<Tree, parser::ParseError> {
    parser::parse(&data)
}

mod parser {
    use crate::{Node, Tree};

    pub type ParseError = peg::error::ParseError<peg::str::LineCol>;

    pub fn parse(s: &str) -> Result<Tree, ParseError> {
        parser::parse(s)
    }

    peg::parser! {
        grammar parser() for str {
            pub rule parse() -> Tree
                = es:edge() ++ eol()
                {
                    let mut tree = Tree::new();
                    es.iter().for_each(|(n1, n2)| {
                        let n1e = tree.entry(n1.clone()).or_insert(std::collections::HashSet::new());
                        n1e.insert(n2.clone());
                        let n2e = tree.entry(n2.clone()).or_insert(std::collections::HashSet::new());
                        n2e.insert(n1.clone());
                    });

                    tree
                }

            rule edge() -> (Node, Node)
                = n1:node() "-" n2:node()
                {
                    (n1, n2)
                }

            rule node() -> Node
                = "start" { Node::Start }
                / "end" { Node::End }
                / l:$(['a'..='z']+) { Node::Small(l.to_string()) }
                / l:$(['A'..='Z']+) { Node::Big(l.to_string()) }

            rule eol()
                = "\n"
                / "\r"
                / "\r" "\n"
        }
    }
}

fn part1(data: &Tree) -> Result<usize, Box<dyn std::error::Error>> {
    let can_traverse_fn = |path: &[Node], node: &Node| !path.contains(node);

    Ok(count_complete_paths(data, can_traverse_fn))
}

fn part2(data: &Tree) -> Result<usize, Box<dyn std::error::Error>> {
    let can_traverse_fn = |path: &[Node], node: &Node| {
        // can always traverse a small node at least once
        if !path.contains(node) {
            return true;
        }

        // count the occurrences in path for each small node
        let mut counts = std::collections::HashMap::<Node, usize>::new();
        path.iter().for_each(|n| match n {
            Node::Small(_) => {
                counts
                    .entry(n.clone())
                    .and_modify(|count| {
                        *count += 1;
                    })
                    .or_insert(1);
            }
            _ => {}
        });

        // if any small node has been visited more than once then we cannot traverse to this node
        !counts.iter().any(|(_, count)| *count > 1)
    };

    Ok(count_complete_paths(data, can_traverse_fn))
}

fn count_complete_paths(data: &Tree, can_traverse_fn: fn(&[Node], &Node) -> bool) -> usize {
    let mut partial_paths = Vec::new();
    let mut complete_paths = Vec::new();

    partial_paths.push(vec![Node::Start]);

    while !partial_paths.is_empty() {
        // get a partial path
        let curr_path = partial_paths.pop().unwrap();

        // get the last node of the selected path
        let curr_node = curr_path.last().unwrap();

        // foreach possible next node (based on the last node)
        data.get(curr_node)
            .unwrap()
            .iter()
            .for_each(|next_node| match next_node {
                Node::Start => {
                    // we do not allow going back to start
                }
                Node::End => {
                    // reaching the end node means completion
                    let mut new_path = curr_path.clone();
                    new_path.push(next_node.clone());

                    complete_paths.push(new_path);
                }
                Node::Big(_) => {
                    // we can always traverse back to a big node
                    let mut new_path = curr_path.clone();
                    new_path.push(next_node.clone());

                    partial_paths.push(new_path);
                }
                Node::Small(_) => {
                    // check if we can traverse back to a small node
                    if can_traverse_fn(&curr_path, next_node) {
                        let mut new_path = curr_path.clone();
                        new_path.push(next_node.clone());

                        partial_paths.push(new_path);
                    }
                }
            });
    }

    complete_paths.len()
}

#[test]
fn test_simulate_sample() {
    let input = aoc::read_file("input/day12.test.txt");
    assert!(input.is_ok());
    let input = input.unwrap();

    let data = parse(&input);
    assert!(data.is_ok());
    let data = data.unwrap();

    let result = part1(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 10);

    let result = part2(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 36);
}

#[test]
fn test_simulate_full() {
    let input = aoc::read_file("input/day12.txt");
    assert!(input.is_ok());
    let input = input.unwrap();

    let data = parse(&input);
    assert!(data.is_ok());
    let data = data.unwrap();

    let result = part1(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 4912);

    let result = part2(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 150004);
}
