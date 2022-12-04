use std::fs;

fn main() {
    let s = fs::read_to_string("./inp/d4.txt").unwrap();

    let mut sum_one = 0u32;
    let mut sum_two = 0u32;
    for line in s.lines() {
        let range_halves = line.split(",").collect::<Vec<_>>();

        let h1 = range_halves[0];

        let h1_bounds = h1.split("-").collect::<Vec<_>>();

        let h1_lower: u32 = h1_bounds[0].parse().unwrap();
        let h1_upper = h1_bounds[1].parse().unwrap();

        let h2 = range_halves[1];
        
        let h2_bounds = h2.split("-").collect::<Vec<_>>();

        let h2_lower = h2_bounds[0].parse().unwrap();
        let h2_upper = h2_bounds[1].parse().unwrap();

        if (h1_lower >= h2_lower && h1_upper <= h2_upper) || (h2_lower >= h1_lower && h2_upper <= h1_upper) {
            sum_one += 1;
        }
        if h1_lower <= h2_upper && h2_lower <= h1_upper {
            sum_two += 1;
        }
    }

    println!("Part 1: {}", sum_one);
    println!("Part 2: {}", sum_two);
}
