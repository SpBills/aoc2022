#![feature(slice_partition_dedup)]

use std::{collections::HashSet, fs};

fn main() {
    let data = fs::read_to_string("./inp/d6.txt").unwrap();
    let window = data
        .as_bytes()
        .windows(4)
        .enumerate()
        .map(|(i, c)| (i + 4, c.iter().collect::<HashSet<_>>().len() == c.len()))
        .find(|(_, b)| *b)
        .unwrap();

    println!("Part 1 found at {}", window.0);

    let window = data
        .chars()
        .collect::<Vec<_>>()
        .windows(14)
        .enumerate()
        .map(|(i, c)| (i + 14, c.iter().collect::<HashSet<_>>().len() == c.len()))
        .find(|(_, b)| *b)
        .unwrap();

    println!("Part 2 found at {}", window.0);
}
