use std::io;

fn fibonacci(n: u32) -> u64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }

    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

fn main() {
    loop {
        println!("Enter a number to compute the Fibonacci term (or type 'exit' to quit):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Trim whitespace and convert to lowercase for case-insensitive comparison
        let trimmed_input = input.trim().to_lowercase();

        if trimmed_input == "exit" {
            break;
        }

        match trimmed_input.parse::<u32>() {
            Ok(n) => println!("The {}th Fibonacci number is: {}", n, fibonacci(n)),
            Err(_) => println!("Please enter a valid integer or 'exit' to quit."),
        }
    }
}
