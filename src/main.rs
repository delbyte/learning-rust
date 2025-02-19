fn is_prime(num: i32) -> bool {
    if num <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    let x = 13;
    if is_prime(x) {
        println!("The number is prime.");
    } else {
        println!("The number is not prime.");
    }
}