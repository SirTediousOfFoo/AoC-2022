use std::{fs, vec};

fn main() {
    let binding = fs::read_to_string("test.input").expect("Unable to read file");
    let lines = binding.lines();

    let mut vector = Vec::new();
    let mut spaces = 0;
    let mut vector2 = Vec::new();

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
    let mut v = vec![];

    for line in lines {
        if line.starts_with("m") {
            v = line.split(" ").filter(|x| 
            x.ne(&"move") && x.ne(&"from") && x.ne(&"to")).collect::<Vec<&str>>();
        }

        if v.len().ne(&0) {
            
            println!("{:?}",v);
        }
            
    }

    println!("{:?}",t);

    println!("{:?}", vector2)
}
