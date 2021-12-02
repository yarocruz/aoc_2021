use std::fs;

/*

forward is horizontal position, in the end the horizontal pos would be 15
down increases depth
up decreases depth

now we would need to track aim

now the result would be to track the aim
 */

fn main() {
    let mut h: i32 = 0;
    let mut aim: i32 = 0;
    let mut depth: i32 = 0;
    let mut result: i32 = 0;

    let file = fs::read_to_string("input.txt").unwrap();
    let course_log = file
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<String>>();

    for i in 0..course_log.len() {
        let dir = course_log[i].split(" ").collect::<Vec<_>>();
        match dir[0] {
            "forward" => {
                h += dir[1].parse::<i32>().unwrap();
                if aim > 0 {
                    depth += aim * dir[1].parse::<i32>().unwrap();
                    result = h * depth;
                }
            },
            "down" => {
                aim += dir[1].parse::<i32>().unwrap();
            },
            "up" => {
                aim -= dir[1].parse::<i32>().unwrap();
            },
            _ => ()
        }
    }
    println!("{}", result);

}
