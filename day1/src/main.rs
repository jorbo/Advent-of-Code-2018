use std::fs;

fn main() {
    let filename = "input.txt";
    let mut freq = 0;
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    for input in contents.lines() {
        let modifer = &input[0..1];
        let value = &input[1..].parse::<i32>().unwrap();
        if modifer == String::from("+"){            
            freq += value;
        }
        else {
            freq -= value;
        }
    }
    println!("{}", freq);    

}
