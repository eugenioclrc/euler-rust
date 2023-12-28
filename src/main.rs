
use std::env;
// #[path = "problems/p1.rs"]
// mod p1;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a number as an argument");
    }
    // Try to parse the second argument as a number
    let number: i32 = args[1].parse()
        .expect("The second argument is not a valid number");
    
    print!("number {}", number);
    
}

