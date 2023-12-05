use std::fs;

fn get_sum_div(num: i32) -> i32 {
    let num_sum: i32 = num.to_string()
        .split("")
        .map(|s| s.trim().parse().unwrap_or(0))
        .sum();

    let divided = num / num_sum;

    // println!("{} {} {}", num, num_sum, divided);

    return divided;
}

fn main() {
    let primes: Vec<i32> = fs::read_to_string("src/someprimes.txt")
        .expect("error")
        .lines()
        .map(|s| s.trim().parse().unwrap_or(0))
        .collect();
    // println!("{:?}", primes);
    
    let max_num: i32 = 100000000;
    // let max_num: i32 = 20;
    let mut num_primes: i32 = 0;
    for i in 1..(max_num + 1) {
        let sum_div: i32 = get_sum_div(i);
        let is_prime = primes.iter().any(|&a| a==sum_div);
        // println!("{}, {}", i, is_prime);
        if is_prime { num_primes += 1 }
    }

    println!("Number of primes: {}", num_primes);
}
