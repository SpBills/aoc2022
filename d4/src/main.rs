use std::fs;

use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    let s = fs::read_to_string("./inp/d4.txt").unwrap();

    let mut sum_1 = 0u32;
    let mut sum_2 = 0u32;
    for line in s.lines() {
        let split_line = line.split(",").collect::<Vec<_>>();

        let a = split_line[0];

        let c = a.split("-").collect::<Vec<_>>();

        let c_0 = c[0].parse::<u32>().unwrap();
        let c_1 = c[1].parse::<u32>().unwrap();

        let b = split_line[1];
        
        let d = b.split("-").collect::<Vec<_>>();

        let d_0 = d[0].parse::<u32>().unwrap();
        let d_1 = d[1].parse::<u32>().unwrap();

        if (c_0 >= d_0 && c_1 <= d_1) || (d_0 >= c_0 && d_1 <= c_1) {
            sum_1 += 1;
        }

        if c_0 <= d_1 && d_0 <= c_1 {
            sum_2 += 1;
        }
    }

    println!("{}", sum_1);
    println!("{}", sum_2);
}
