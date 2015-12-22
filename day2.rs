use std::io;
use std::io::BufRead;

fn main() {
    let mut paper = 0;
    let mut ribbon = 0;

    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let mut fields : Vec<u32> = line.split('x')
                                        .map(|s| s.parse::<u32>().unwrap())
                                        .collect();

        assert_eq!(fields.len(), 3);

        fields.sort();
        let (l, w, h) = (fields[0], fields[1], fields[2]);

        let surface = 2*l*w + 2*w*h + 2*h*l;
        let slack = l*w; // because of sorting, l and w are the smallest values
        paper += surface + slack;

        let wrap = 2*l + 2*w; // need smallest values again
        let bow = l*w*h;
        ribbon += wrap + bow;
    }

    println!("paper={}, ribbon={}", paper, ribbon);
}
