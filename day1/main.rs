use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn main() {
    let contents = lines_from_file("puzzle.input");
    let mut _biggest = 0;
    let mut sum = 0;
    let mut elf = 1;
    let mut biggest_elf = elf;
    let mut top3 = vec![0,0,0];
    let stopper = contents.len() + 1;

    for row in contents {
        
        if row.len() != 0 {
            sum = sum + row.parse::<i32>().unwrap();
        }
        else if row.len() == 0 || elf >= stopper{
            if sum >= top3[0] {
                top3[0] = sum;
            }
            top3.sort();
            sum = 0;
            elf = elf+1;
        }
    }
    if sum >= top3[0] {
        top3[0] = sum;
    }
    top3.sort();
    let top3sum: i32 = top3.iter().sum();
     println!("Part 1: Elf {biggest_elf} is carrying {:?} calories!",top3[2]);
     println!("Part 2: The sum of the top 3 calorie carrying elves is {top3sum}!");
}