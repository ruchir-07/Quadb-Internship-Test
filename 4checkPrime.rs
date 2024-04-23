use std::io;

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number: u32 = input.trim().parse().expect("Not an integer!");
    println!("Is {} a prime number? {}", number, is_prime(number));
}
