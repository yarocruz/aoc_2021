use std::fs::File;
use std::io::Read;

/*
List of measurements like:
199
200
201
202
200

Add a counter that keeps track of the increases.

If the “next” number is > than “previous”
Increase the counter.

We would also need to track the previous

1. Create a binding to store previous number
2. Create the counter binding to keep track of increases
3. Read from file and for each line, check if number from first line is less than next line
4. Add first number to the previous greatest variable.
5. If it is, increase the counter and add the number to previous greatest
 */

fn main() {
    let mut previous: i32 = 0;
    let mut n_increases: i32 = 0;

    // read from file
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();

    // we need to make the contents of the file to a string
    file.read_to_string(&mut contents).unwrap();

    let mut v: Vec<i32> = Vec::new();
    // push each line from the contents of the file in vector as a number i32
    for s in contents.lines() {
        v.push(s.parse::<i32>().unwrap());
    }

    // store first number in vector before starting loop
    previous = v[0];

    // loop through the vector
    for i in 1..v.len() {
      //println!("{} {}", previous, v[i]);
      if previous < v[i] {
          n_increases += 1;
          //println!("{}", n_increases);
      }
      // update previous
        previous = v[i];
    }
    println!("{}", n_increases);
}


