use std::io;
use hex;

fn main() {

    println!("Input a hex color code");

    let mut hex_code = String::new();

    io::stdin()
        .read_line(&mut hex_code)
        .expect("Failed to read line");

    println!("Hex: {hex_code}"); 

    // Convert String to array of chars  
    let letters: Vec<char> = hex_code.chars().collect();

    let array: [&str; 3] = ["R: ", "G: ", "B: "];
    for index in 0..3 {
        let chars = &hex_code[(index*2)..{index*2+1}];
        let hex_str = match hex::decode(chars) {
            Ok (num) => num,
            Err (_) => vec![0; 1],
        };
        println!("{}: {:?}", array[index], hex_str ); 
    }
}
