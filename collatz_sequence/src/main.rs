/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut count: u32 = 1;
    while n > 1 {
        // n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };

        n = match n % 2 {
            0 => n / 2,
            _ => 2 * n + 1,
        };
        count += 1;
    }

    count
}

fn main() {
    println!("Length: {}", collatz_length(11)); // should be 15
}
