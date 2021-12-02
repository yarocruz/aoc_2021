use std::fs;

/*
forward 5
down 5
forward 8
up 3
down 8
forward 2

forward is horizontal position, in the end the horizontal pos would be 15
down increases depth
up decreases depth

in the end we multiply the total depth * horizontal pos to get result

 */

fn main() {
    let mut h: i32 = 0;
    let mut depth: i32 = 0;
    let mut result: i32 = 0;

    let file = fs::read_to_string("input.txt").unwrap();
    let course_log = file
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<String>>();

    // There's probably a better way to do this
    for i in 0..course_log.len() {
        let dir = course_log[i].split(" ").collect::<Vec<_>>();
        match dir[0] {
            "forward" => h += dir[1].parse::<i32>().unwrap(),
            "down" => depth += dir[1].parse::<i32>().unwrap(),
            "up" => depth -= dir[1].parse::<i32>().unwrap(),
            _ => ()
        }
        //println!("{}{}", h, depth);
    }
    result = h * depth;
    println!("{}", result = h * depth);

}
