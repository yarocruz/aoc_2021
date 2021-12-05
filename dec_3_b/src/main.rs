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

This one is going to be life_support rating = ox_gen_rating * c02_scrub_rating

This time we consider just the first bit

 */

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut bytes = file
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<String>>();

    let power_consumption:isize;

    let mut ones_at_pos: [i32; 12] = [0; 12];
    let mut zeroes_at_pos: [i32; 12] = [0; 12];

    let mut ox_gen_rating: Vec<_> = bytes.clone();
    let mut c02_scrub_rating: Vec<_> = bytes.clone();

    while ox_gen_rating.len() > 1 {
        for i in 0..11 {
            if ones_at_pos[i] >= zeroes_at_pos[i] {
                // then filter out the byte vector by all the strings that start 0
                if ox_gen_rating.len() > 1 {
                    ox_gen_rating = ox_gen_rating.into_iter().filter(|x| x.chars().nth(i).unwrap() == '1').collect::<Vec<_>>();
                }

                println!("{:?}", ox_gen_rating);
                println!("{:?}", ox_gen_rating.len());
            } else if ones_at_pos[i] < zeroes_at_pos[i] && ox_gen_rating.len() > 1 {
                ox_gen_rating = ox_gen_rating.into_iter().filter(|x| x.chars().nth(i).unwrap() == '0').collect::<Vec<_>>();
            }

            //reset
            zeroes_at_pos = [0,0,0,0,0,0,0,0,0,0,0,0];
            ones_at_pos = [0,0,0,0,0,0,0,0,0,0,0,0];
            for k in 0..ox_gen_rating.len() {
                let bits: Vec<char> = ox_gen_rating[k].chars().collect();

                for j in 0..bits.len() {
                    if bits[j] == '0' {
                        zeroes_at_pos[j] += 1;
                    } else if bits[j] == '1' {
                        ones_at_pos[j] += 1;
                    }
                }
            }
        }


    }

    while c02_scrub_rating.len() > 1 {
        for i in 0..11 {
            if ones_at_pos[i] < zeroes_at_pos[i] {
                // then filter out the byte vector by all the strings that start 0
                if c02_scrub_rating.len() > 1 {
                    c02_scrub_rating = c02_scrub_rating.into_iter().filter(|x| x.chars().nth(i).unwrap() == '1').collect::<Vec<_>>();
                }

                println!("{:?}", c02_scrub_rating);
                println!("{:?}", c02_scrub_rating.len());
            } else if ones_at_pos[i] >= zeroes_at_pos[i] && c02_scrub_rating.len() > 1 {
                c02_scrub_rating = c02_scrub_rating.into_iter().filter(|x| x.chars().nth(i).unwrap() == '0').collect::<Vec<_>>();
            }
            //reset
            zeroes_at_pos = [0,0,0,0,0,0,0,0,0,0,0,0];
            ones_at_pos = [0,0,0,0,0,0,0,0,0,0,0,0];

            for k in 0..c02_scrub_rating.len() {
                let bits: Vec<char> = c02_scrub_rating[k].chars().collect();

                for j in 0..bits.len() {
                    if bits[j] == '0' {
                        zeroes_at_pos[j] += 1;
                    } else if bits[j] == '1' {
                        ones_at_pos[j] += 1;
                    }
                }
            }
        }
    }


    let ox = isize::from_str_radix(&ox_gen_rating[0], 2).unwrap();
    let co2 = isize::from_str_radix(&c02_scrub_rating[0], 2).unwrap();

    power_consumption = ox * co2;
    //
    println!("{:?}{:?}", ox_gen_rating, c02_scrub_rating);
    println!("Final Result {}", power_consumption);
}

