
use std::env;
// #[path = "problems/pN.rs"]
// mod pN;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a number as an argument");
    }
    // Try to parse the second argument as a number
    let number: i32 = args[1].parse()
        .expect("The second argument is not a valid number");
    
        print!("number {}/n", number);
        let mut s:u64 = 0;
        for j in 1..number {
            let f:u64 = fibonacci2(j).expect("not a number");
            if ((f%2) == 0) {
                s += f;
            }
        }

        print!("number sum even {:?}", s);
    
}

fn fibonacci(n: i32) -> u64 {
    if n == 0 {
        panic!("n must be greater than 0")
    } else if n == 1 {
        return 0;
    } else if n == 2 {
        return 1;
    } else {
        return fibonacci(n -1) + fibonacci(n - 2)
    }
}


fn fibonacci2(n: i32) -> Option<u64> {
    if n <= 0 {
        return None;
    } else if n == 1 {
        return Some(0);
    } else if n == 2 {
        return Some(1);
    }
    let mut a = 0;
    let mut b = 1;
    let mut sum = 0;

    for _ in 2..n {
        sum = a + b;
        a = b;
        b = sum;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fibonacci() {
        assert_eq!(0, super::fibonacci(1));
        assert_eq!(1, super::fibonacci(2));
        assert_eq!(1, super::fibonacci(3));
        assert_eq!(2, super::fibonacci(4));
    }

    #[test]
    fn test_fibonacci2() {
        assert_eq!(0, super::fibonacci2(1).unwrap());
        assert_eq!(1, super::fibonacci2(2).unwrap());
        assert_eq!(1, super::fibonacci2(3).unwrap());
        assert_eq!(2, super::fibonacci2(4).unwrap());
    }
}