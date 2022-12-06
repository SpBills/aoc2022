#![feature(slice_partition_dedup)]

use std::fs;

fn main() {
    let data = fs::read_to_string("./inp/d6.txt").unwrap();
    let window = data
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .enumerate()
        .map(|(i, c)| {
            let mut p = c.to_owned();
            p.sort();
            let part = p.partition_dedup();
            dbg!(&part);
            (i+4, part.1.len() == 0)
        })
        .filter(|(_, b)| !!b)
        .collect::<Vec<_>>();

    dbg!(window[0]);
}
