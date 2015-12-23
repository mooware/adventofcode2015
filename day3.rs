use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn main() {
    let mut x = 0;
    let mut y = 0;
    let mut set : HashSet<(i32, i32)> = HashSet::new();
    set.insert((x, y));

    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        for c in line.chars() {
            match c {
                '<' => x -= 1,
                '>' => x += 1,
                'v' => y -= 1,
                '^' => y += 1,
                _ => panic!("unexpected input '{}'", c),
            }

            set.insert((x, y));
        }
    }

    println!("count={}", set.len());
}
