use std::fs;

// n collections of m characters
#[derive(Debug)]
struct Rucksack(Vec<Vec<char>>);

fn split_nth(ruck_str: String, n: usize) -> Rucksack {
    let slice = ruck_str.chars().collect::<Vec<_>>();
    let len = slice.len();
    assert!(len >= n);
    let (quo, rem) = (len / n, len % n);
    let split = (quo + 1) * rem;
    let c = slice[..split]
        .chunks(quo + 1)
        .chain(slice[split..].chunks(quo))
        .map(|s| s.iter().cloned().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    return Rucksack(c);
}

fn main() {
    let all_rucksacks = fs::read_to_string("./inp/d3.txt").unwrap();

    let sacks = all_rucksacks
        .lines()
        .map(|r| split_nth(r.to_owned(), 2))
        .collect::<Vec<Rucksack>>();

    dbg!(sacks);
}
