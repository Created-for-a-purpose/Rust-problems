// Implement a function that checks whether a given number is prime or not.

fn is_prime(num: u32) -> bool {
    if num<=1 {
        return false;
    }

    let sqrt_num = (num as f64).sqrt() as u32;
    for i in 2..=sqrt_num {
        if num%i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let number: u32 = 11;
    let isPrime: bool = is_prime(number);
    println!("Number {} is prime: {}", number, isPrime);
}