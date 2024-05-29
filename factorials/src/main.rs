// Factorial excersize

fn main() {
    for n in 0..22 {
        println!("{}! = {}", n, factorial(n));
    }
}

fn factorial(n: i64) -> i64 {

    assert!(n >= 0, "n must be positive");
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
