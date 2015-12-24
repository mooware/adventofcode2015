use std::io;
use std::io::BufRead;

fn main() {
    let badstrings = ["ab", "cd", "pq", "xy"];

    let mut nice = 0;
    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let mut repeat = 0;
        let mut vowels = 0;

        let mut prev = '\0';
        for c in line.chars() {
            if "aeiou".contains(c) { vowels += 1; }
            if c == prev { repeat += 1; }
            prev = c;
        }

        let mut bads = false;
        for s in badstrings.iter() {
            if line.contains(s) {
                bads = true;
                break;
            }
        }

        if vowels >= 3 && repeat > 0 && !bads {
            nice += 1;
        }
    }

    println!("nice={}", nice);
}
