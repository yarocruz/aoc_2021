use std::fs;

/*

params:

power_consumption

gamma_ray and epsilon_rate

gamma_ray = most common bit

0 0 1 0 0
1 1 1 1 0
1 0 1 1 0
1 0 1 1 1
1 0 1 0 1
0 1 1 1 1
0 0 1 1 1
1 1 1 0 0
1 0 0 0 0
1 1 0 0 1
0 0 0 1 0
0 1 0 1 0

For each five bit sequence, we are going to compare the bits
starting from left to right. So in the first col, (first bits)
7 1's and 5 0', so 1 is the most common.

The trick is to count how many 1's and 0's at a particular index

1. So for each col, count the ones and zeros.
2. save them to a least_common and / or most_common binding
3. save the most common bit to the gamma_ray
5. save the least common bit to the epsilon
6. result = gamma_ray * epsilon_rate

 */

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let bytes = file
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<String>>();

    println!("{:?}", bytes);

    let mut gamma_ray = String::new();
    let mut epsilon = String::new();

    let power_consumption:isize;

    let mut ones_at_pos: [i32; 12] = [0; 12];
    let mut zeroes_at_pos: [i32; 12] = [0; 12];

    for i in 0..bytes.len() {
        let bits: Vec<char> = bytes[i].chars().collect();

        for j in 0..bits.len() {
            if bits[j] == '0' {
                zeroes_at_pos[j] += 1;
            } else if bits[j] == '1' {
                ones_at_pos[j] += 1;
            }
        }
    }

    for i in 0..ones_at_pos.len() {
        if ones_at_pos[i] > zeroes_at_pos[i] {
            gamma_ray += "1";
            epsilon += "0";
        } else {
            gamma_ray += "0";
            epsilon += "1";
        }
    }

    println!("Gamma Ray : {}, Epsilon: {}", gamma_ray, epsilon);

    println!("{}:{}", ones_at_pos[11], zeroes_at_pos[11]);

    let gr = isize::from_str_radix(&gamma_ray, 2).unwrap();
    let ep = isize::from_str_radix(&epsilon, 2).unwrap();

    power_consumption = gr * ep;

    println!("Final Power Consumption {}", power_consumption);
}
