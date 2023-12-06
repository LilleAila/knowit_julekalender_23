// use std::fs;

fn get_sum_div(num: i32) -> i32 {
    let num_sum: i32 = num.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum::<u32>() as i32;

    let divided: f32 = num as f32 / num_sum as f32;

    // println!("{} {} {}", num, num_sum, divided);

    if divided % 1.0 > 0.0 { return -1 }

    return divided as i32;
}

// fn is_prime(n: i32) -> bool {
//     if n <= 1 { // Hvis 1 er et primtall, det er det sikkert ikke
//         return false;
//     }
//     if n <= 3 {
//         return true;
//     }
//     if n % 2 == 0 || n % 3 == 0 {
//         return false;
//     }
//
//     let mut i = 5;
//     while i * i <= n {
//         if n % i == 0 || n % (i + 2) == 0 {
//             return false;
//         }
//         i += 6;
//     }
//
//     true
// }

fn is_prime (n: i32) -> bool {
    if n == 2 { return true }
    if n % 2 == 0 { return false }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
    // !(2..(n as f32).sqrt().ceil() as i32).any(|a| n % a == 0)
}

fn check_prime_div(num: i32) -> bool {
    let sum_div: i32 = get_sum_div(num);
    if sum_div < 0 { return false };
    return is_prime(sum_div);
}

fn main() {
    let min_num: i32 = 1;
    let max_num: i32 = 100000000;
    // let max_num: i32 = 100;
    let mut num_primes: i32 = 0;
    for i in (min_num)..(max_num + 1) {
        let checked: bool = check_prime_div(i);
        if checked {
            num_primes += 1;
            // println!("{}", i);
        }
        if i % 5000000 == 0 { println!("{}", i) }
    }

    println!("Antall sum-dele-primtall: {}", num_primes);
    // 6k±1 prime alg: 1020748
    // One-line from SO: 1022242
    // Simple: 1020757

    // println!("{}", check_prime_div(9015));
}
