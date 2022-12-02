use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let binding = fs::read_to_string("puzzle.input").expect("Unable to read file");
    let lines = binding.lines();
    let mut score = 0;
    for line in lines {
        let pair: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
        if pair[1] == "X" {
            score = score + 1;
            if pair[0] == "C" {
                score = score + 6;
            }
            if pair[0] == "A" {
                score = score + 3;
            }

        } 
        if pair[1] == "Y" {
            score = score + 2;
            if pair[0] == "A" {
                score = score + 6;
            }
            if pair[0] == "B" {
                score = score + 3;
            }

        } 
        if pair[1] == "Z" {
            score = score + 3;
            if pair[0] == "B" {
                score = score + 6;
            }
            if pair[0] == "C" {
                score = score + 3;
            }

        } 
    }
    println!("{score}")
}

fn part2() {
    let binding = fs::read_to_string("puzzle.input").expect("Unable to read file");
    let lines = binding.lines();
    let mut score = 0;
    for line in lines {
        let pair: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
        if pair[1] == "X" {
            if pair[0] == "A" {
                score = score + 3;
            }
            if pair[0] == "B" {
                score = score + 1;
            }
            if pair[0] == "C" {
                score = score + 2;
            }
//            println!("{score}")

        } 
        if pair[1] == "Y" {
            if pair[0] == "A" {
                score = score + 4;
            }
            if pair[0] == "B" {
                score = score + 5;
            }
            if pair[0] == "C" {
                score = score + 6;
            }
 //           println!("{score}")

        } 
        if pair[1] == "Z" {
            if pair[0] == "A" {
                score = score + 8;
            }
            if pair[0] == "B" {
                score = score + 9;
            }
            if pair[0] == "C" {
                score = score + 7;
            }
//            println!("{score}")

        } 
    }
    println!("{score}")
}