use std::io;
use std::io::Write;

fn main() {
    // Initialize the prefilled vector.
    let prefilled_values = prefill_vector();

    // Create a vector for fill-on-the-fly.
    let mut fill_on_the_fly_values: Vec<i64> = vec![0, 1];

    loop {
        // Prompt the user for n.
        let n = get_i64("N: ");

        // add to get clean exit.
        if n < 0 {
            break
        }

        // Calculate the Fibonacci number.
        println!("Prefilled:  {}", prefilled_values[n as usize]);
        println!("On the fly: {}", fibonacci_on_the_fly(&mut fill_on_the_fly_values, n));
        println!("Bottom up:  {}", fibonacci_bottom_up(n));
        println!();
    }
}

// calculate the Fibonacci number uses pre-calaculated values. If value is not
// in the vector, calculate it and add it to the vector.
fn fibonacci_on_the_fly(values: &mut Vec<i64>, n: i64) -> i64 {
    if n >= values.len() as i64 {
        // had to use seperate calculation to avoid borrow checker error.
        let a = fibonacci_on_the_fly(values, n - 1);
        let b = fibonacci_on_the_fly(values, n - 2);
        values.push(a + b);            
    }
    return values[n as usize];    
}


// Prefill the vector with the first 93 Fibonacci numbers.
fn prefill_vector() -> Vec<i64> {
    let mut values: Vec<i64> = vec![0, 1];

    for i in 2..=92 {
        values.push(values[i - 1] + values[i - 2]);
    }
    return values;
}


// instructor solution -> bottom up approach to calculation.
fn fibonacci_bottom_up(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }

    let mut fib_i_minus_2 = 0i64;
    let mut fib_i_minus_1 = 1i64;
    let mut fib_i = fib_i_minus_1 + fib_i_minus_2;
    for _ in 1i64..n {
        // Calculate this value of fib_i.
        fib_i = fib_i_minus_1 + fib_i_minus_2;

        // Set fib_i_minus_2 and fib_i_minus_1 for the next value.
        fib_i_minus_2 = fib_i_minus_1;
        fib_i_minus_1 = fib_i;
    }
    return fib_i;
}

// Prompt the user for an i64.
fn get_i64(prompt: &str) -> i64 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    return trimmed.parse::<i64>()
        .expect("Error parsing integer");
}
