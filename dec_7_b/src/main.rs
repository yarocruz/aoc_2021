use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    println!("{}", process(&file));
}

fn process(input: &str) -> i64 {
    let crabs: Vec<i64> = input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    let mut best = i64::MAX;

    for meeting_point in min..=max {
        let mut score = 0;
        for crab in &crabs {
            let delta = (crab - meeting_point).abs();
            score += delta * (delta + 1) / 2;
        }

        best = best.min(score)
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &'static str = include_str!("./test-input.txt");

    #[test]
    fn test_demo_data() {
        assert_eq!(168, process(input));
    }
}
