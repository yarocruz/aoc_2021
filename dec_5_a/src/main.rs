use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process(&file, true));
}

fn process(input: &str, consider_diag: bool) -> i32 {
    // we're going to store the number of intersecting lines in a HashMap
    let mut int_lines: HashMap<(i32, i32), i32> = HashMap::new();

    // basic parsing of the input
    for line in input.trim().split('\n') { // first line ex. "0,9 -> 5,9"
        let mut cords = line.split(" -> "); // cords is an iterator which is an object with the next method right?
        // we store just the coord nums in vecs
        let start: Vec<i32> = cords
            .next() // <-- we get '0, 9' for that first next
            .unwrap()
            .split(',') // here is were we get ['0', '9'] here
            .map(|n| n.parse().unwrap())
            .collect();

        let end: Vec<i32> = cords
            .next() // <-- in this next we get '5, 9' for that first next
            .unwrap()
            .split(',') // here is were we get ['5', '9'] here
            .map(|n| n.parse().unwrap())
            .collect();

        // from the vecs we go those x and y values to do the comparison
        let x0 = start[0];
        let y0 = start[1];
        let x1 = end[0];
        let y1 = end[1];

        if x0 == x1 {
            for y in y0.min(y1)..=y0.max(y1) { // the min and max methods are there to make sure we cover when cords go 'down' vs up (eg. 9, 4 to 3, 4)
                *int_lines.entry((x0, y)).or_default() += 1;
            }
        } else if y0 == y1 {
            for x in x0.min(x1)..=x0.max(x1) { // the min and max methods are there to make sure we cover when cords go 'down' vs up (eg. 9, 4 to 3, 4)
                *int_lines.entry((x, y0)).or_default() += 1;
            }
        } else if consider_diag {
            let dx = if x1 - x0 > 0 { 1 } else { -1 };
            let dy = if y1 - y0 > 0 { 1 } else { -1 };

            let mut x = x0;
            let mut y = y0;

            *int_lines.entry((x, y)).or_default() += 1;

            while x != x1 {
                x += dx;
                y += dy;
                *int_lines.entry((x, y)).or_default() += 1;
            }
            assert!(y == y1);
        }

    }
    int_lines.values().filter(|f| **f >= 2).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &'static str = include_str!("./test-input.txt");

    #[test]
    fn test_demo_data() {
        assert_eq!(5, process(input));
    }
}
