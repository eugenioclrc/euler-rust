/**
 * Problem 1: Multiples of 3 and 5
 * If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6
 * and 9. The sum of these multiples is 23.
 * Find the sum of all the multiples of 3 or 5 below the provided parameter value number.
 */

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a number as an argument");
    }
    // Try to parse the second argument as a number
    let number: i32 = args[1].parse()
        .expect("The second argument is not a valid number");
    
    print!("The sum of all the multiples of 3 or 5 below {} is {}", number, multiplesOf3and5(number));
    
}

 #[allow(non_snake_case)]
 fn multiplesOf3and5(number: i32) -> i32 {
    let mut sum = 0;
    for i in 1..number {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

#[allow(non_snake_case)]
fn multiplesOf3and5Short(number: i32) -> i32 {
    return (1..number).filter(|i| i % 3 == 0 || i % 5 == 0).sum();
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_multiplesof3and5() {
        // multiplesOf3and5(49) should return 543.
        assert_eq!(543, super::multiplesOf3and5(49));
        // multiplesOf3and5(1000) should return 233168.
        assert_eq!(233168, super::multiplesOf3and5(1000));
        // multiplesOf3and5(8456) should return 16687353.
        assert_eq!(16687353, super::multiplesOf3and5(8456));
        // multiplesOf3and5(19564) should return 89301183.
        assert_eq!(89301183, super::multiplesOf3and5(19564));
    }

    #[test]
    fn test_multiplesof3and5short() {
        // multiplesOf3and5Short(49) should return 543.
        assert_eq!(543, super::multiplesOf3and5Short(49));
        // multiplesOf3and5Short(1000) should return 233168.
        assert_eq!(233168, super::multiplesOf3and5Short(1000));
        // multiplesOf3and5Short(8456) should return 16687353.
        assert_eq!(16687353, super::multiplesOf3and5Short(8456));
        // multiplesOf3and5Short(19564) should return 89301183.
        assert_eq!(89301183, super::multiplesOf3and5Short(19564));
    }
}