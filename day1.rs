use std::env;
use std::io;
use std::io::Read;

fn main() {
    // use arg no. 1 or stdin
    let mut input = String::new();
    let arg = env::args().nth(1);
    if arg.is_some() {
        input = arg.unwrap();
    } else {
        io::stdin().read_to_string(&mut input).unwrap();
    }

    let mut count = 0;
    let mut basement = 0;

    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            count += 1;
        } else if c == ')' {
            count -= 1;
        }

        if basement == 0 && count == -1 {
            basement = i + 1; // result is one-based
        }
    }

    println!("count={}, basement={}", count, basement);
}
