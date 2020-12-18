use rand::{thread_rng, Rng};
use std::collections::HashMap;

const SIZE: usize = 8;

fn main() {
    let numbers: [u8; SIZE] = thread_rng().gen();
    let mut vector = numbers.to_vec();
    let mut sum: usize = 0;
    let mut values = HashMap::new();

    vector.sort();

    for &n in &vector {
        sum += usize::from(n);
        let count = values.entry(n).or_insert(0);
        *count += 1;
    }

    let mean = sum / vector.len();
    let median = vector[(vector.len() - 1) / 2];

    // println!("{:?}", vector);
    println!("Sum: {}, Average: {}, Median: {}", sum, mean, median);
    println!("Mode:{:?}", values);
}
