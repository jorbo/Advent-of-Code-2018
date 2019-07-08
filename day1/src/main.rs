use std::fs;

fn main() {
    let filename = "input.txt";   
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    // Reworked more rust-like solution
    let freq_list: Vec<i32> = contents.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let freq: i32 = freq_list.iter().sum();
    
    
    // Babies first rust basiclly could be language agnostic
    // for input in contents.lines() {

    //     // Get the + or - mod
    //     let modifier = &input[0..1];

    //     // change our string from the file into an int so we can make it do math
    //     let value = &input[1..].parse::<i32>().unwrap();
        
    //     // if this modifier is the + char add the value if its anything else subtract the value
    //     if modifier == String::from("+"){            
    //         freq += value;
    //     }
    //     else {
    //         freq -= value;
    //     }

    // }    

    // Ding ding the answer!
    println!("{}", freq);    

}
