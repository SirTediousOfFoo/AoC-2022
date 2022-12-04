use std::{fs, collections::HashMap};

fn main() {
    let binding = fs::read_to_string("puzzle.input").expect("Unable to read file");
    let lines = binding.lines();
    let mut value = 1;
    let mut alphabet = HashMap::new();
    let mut score = 0;
    let mut flag = false;

    for x in 'a'..='z' {
        alphabet.insert(x, value);
        value = value + 1;    
    }
    for x in 'A'..='Z' {
        alphabet.insert(x, value);
        value = value + 1;    
    }
    for line in lines.clone() {
        for i in 0..line.len()/2 {
            for j in line.len()/2..line.len() {
                if line.chars().nth(j) == line.chars().nth(i) {
                    flag = true;
                }
            }
            if flag == true {
                score = score + alphabet.get(&line.chars().nth(i).unwrap().into()).copied().unwrap();
                break;
            }
        }
        flag = false;
        println!("{}",score)
    }

    score = 0;
    let lines_str: Vec<String> = lines.map(|x| x.to_string()).collect();
        for truples in lines_str.chunks(3) {
            for character in truples[0].chars() {
                if truples[1].contains(character) && truples[2].contains(character) {
                    flag == true;
                    score = score + alphabet.get(&character).copied().unwrap();
                    break;
                }
            }
        }
    
    println!("{score}");
    //println!("{:?}",lines.take(3));
}
