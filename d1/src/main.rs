use std::fs;

fn main() {
    let unparsed_data = fs::read_to_string("./inp/d1.txt").unwrap();

    let mut sums: Vec<usize> = vec![];
    let mut i = 0;
    for line in unparsed_data.lines() {
        if line.is_empty() {
            i += 1;
            continue;
        }

        if let None = sums.get(i) {
            sums.push(line.parse::<usize>().unwrap());
            continue;
        }

        sums[i] = sums[i] + line.parse::<usize>().unwrap();
    }

    sums.sort_by(|a, b| b.cmp(a));
    println!("{}", sums[0] + sums[1] + sums[2]);
}
