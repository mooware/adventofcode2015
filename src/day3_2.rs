use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn main() {
    let mut santa = (0, 0);
    let mut robosanta = (0, 0);
    let mut is_robo = false;
    let mut set : HashSet<(i32, i32)> = HashSet::new();
    set.insert(santa);

    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        for c in line.chars() {
            let pos = if is_robo { &mut robosanta } else { &mut santa };
            is_robo = !is_robo;

            let (x, y) = *pos;

            *pos = match c {
                '<' => (x-1, y),
                '>' => (x+1, y),
                'v' => (x, y-1),
                '^' => (x, y+1),
                _ => panic!("unexpected input '{}'", c),
            };

            set.insert(*pos);
        }
    }

    println!("count={}", set.len());
}
