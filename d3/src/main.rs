use std::fs;

#[derive(Debug, Clone, Copy)]
struct Valuable(char, u32);

impl Valuable {
    fn new(c: char) -> Self {
        if c.is_uppercase() {
            return Self(c, c as u32 - 38)
        } else {
            return Self(c, c as u32 - 96)
        }
    }
}

// n collections of m characters
#[derive(Debug)]
struct Rucksack(Vec<Vec<Valuable>>);

impl Rucksack {
    fn find_dup_sum(&self) -> Option<Valuable> {
        let first = &self.0[0];
        let second = &self.0[1];

        for x in first {
            for y in second {
                if x.0 == y.0 {
                    return Some(x.clone());
                }
            }
        }

        None
    }
}

fn split_nth(ruck_str: String, n: usize) -> Rucksack {
    let slice = ruck_str.chars().collect::<Vec<_>>();
    let len = slice.len();
    assert!(len >= n);
    let (quo, rem) = (len / n, len % n);
    let split = (quo + 1) * rem;
    let c = slice[..split]
        .chunks(quo + 1)
        .chain(slice[split..].chunks(quo))
        .map(|s| s.iter().map(|c| Valuable::new(*c)).collect::<Vec<Valuable>>())
        .collect::<Vec<Vec<Valuable>>>();

    return Rucksack(c);
}

fn main() {
    let all_rucksacks = fs::read_to_string("./inp/d3.txt").unwrap();

    let sacks = all_rucksacks
        .lines()
        .map(|r| split_nth(r.to_owned(), 2))
        .collect::<Vec<Rucksack>>();

    let sum: u32 = sacks.iter().map(|s| s.find_dup_sum().unwrap().1).sum();

    println!("{}", sum);
}
