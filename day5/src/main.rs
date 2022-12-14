use std::{fs, vec};

fn main() {
    let binding = fs::read_to_string("puzzle.input").expect("Unable to read file");
    let lines = binding.lines();

    let mut vector = Vec::new();
    let mut spaces = 0;
    let mut vector2 = Vec::new();
    let mut fin = Vec::new();

    for line in lines.clone() {

        vector = [].to_vec();

        if line.contains('1') {
            break;
        }

        for c in line.chars() {
            if c.eq(&' ') {
                spaces = spaces + 1;
                if spaces == 4 {
                    vector.push(' ');
                    spaces = 0;
                }
            }
            if c.is_alphabetic() {
                spaces = 0;
                vector.push(c);
            }
        }

        vector2.push(vector)
    }

     let x = 0;
    let mut t = vec![Vec::with_capacity(vector2.len()); vector2[0].len()];
    for r in vector2.to_owned() {
        for i in 0..r.len() {
            t[i].push(r[i]);
        }
    }
    
    for i in 0..vector2[0].len() {
        vector = [].to_vec();

        for member in &vector2{
            if member[i].ne(&' ') {
                vector.push(member[i]);
            } 
        }
        fin.push(vector);
            vector = [].to_vec();
    }
    
    let mut v = vec![];

    for line in lines {
        if line.starts_with("m") {
            v = line.split(" ")
                .filter(|x| 
                    x.ne(&"move") && x.ne(&"from") && x.ne(&"to"))
                .collect::<Vec<&str>>();
        }

        if v.len().ne(&0) {
          
            for i in 0..(v[0].to_string().parse::<i32>().unwrap()) {

                let c = fin[v[1].to_string().parse::<usize>().unwrap()-1][0];

                fin[v[2].to_string().parse::<usize>().unwrap()-1].insert(0, c);

                fin[v[1].to_string().parse::<usize>().unwrap()-1].remove(0);
            }
        }
            
    }

    let mut output = Vec::new();

    for thing in fin {
        output.push(thing[0]);
    }

    println!("{:?}", output);
}
