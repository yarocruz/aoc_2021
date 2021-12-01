use std::fs::File;
use std::io::Read;

/*
Add a counter that keeps track of the 'sliding window increases'.

We would iterate and add the sum of first and next 'threes' and add
that value into a binding to keep track of
We would also need to track the previous

1. Create a binding to store previous
2. Create the counter binding to keep track of increases
3. Read from file and for each line, check if number from first line is less than next line
 */

fn main() {
    //let mut previous: i32 = 0;
    let mut next_3_additions: i32 = 0;
    let mut prev_value: i32 = 0;
    let mut increases_counter: i32 = 0;

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
    // loop through the vector

    for i in 0..v.len() {
        /*

        Add the first three
        Put result in addition variable
        compare that addition the prev increase

        if addition is greater than a prev addition, increase counter that tracks those increases

        then add the next three, and so on..

        We just need to make sure to not go out of bounds
         */

        // we make sure to not go out of bounds with
        // the index when doing the addition of next three
        if i < v.len() - 3 {
            next_3_additions = v[i] + v[i + 1] + v[i + 2];
        }

        if next_3_additions > prev_value {
            increases_counter += 1;
        }
        // we update here prev value to be able to use it to check on next [i]teration
        prev_value = next_3_additions;
    }
    println!("{}", increases_counter);
}


