use std::{fs};

fn main() {
    let binding = fs::read_to_string("puzzle.input").expect("Unable to read file");
    let lines = binding.lines();
    let mut counter_full = 0;
    let mut counter_part = 0;

    for line in lines {
        let pairs = line.split_once(",").unwrap();
        let a1 = pairs.0.split_once("-").unwrap();
        let a2 = pairs.1.split_once("-").unwrap();
        let x: i32= a1.0.parse().unwrap();
        let y: i32 = a1.1.parse().unwrap();
        let a: i32 = a2.0.parse().unwrap();
        let b: i32 = a2.1.parse().unwrap();
        
        if x.le(&a) && y.ge(&a) && y.le(&b) || a.le(&x) && b.ge(&x) && y.le(&b) {
            counter_full = counter_full + 1;
        }   

        if (a..=b).contains(&y) || (a..=b).contains(&x) || (x..=y).contains(&a) || (x..=y).contains(&b) {
            counter_part = counter_part + 1;
        } 
    }

    println!("Full overlap: {counter_full}, partial overlap: {counter_part}");
}
