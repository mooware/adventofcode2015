extern crate pcre;
use pcre::Pcre;

use std::io;
use std::io::BufRead;

fn main() {
    let nicepatterns = [r"([[:alpha:]]{2}).*\1", r"([[:alpha:]]).\1"];
    let mut nicere = nicepatterns.iter()
                                 .map(|&s| Pcre::compile(s).unwrap())
                                 .collect::<Vec<_>>();

    let mut nice = 0;
    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        if nicere.iter_mut().all(|r| r.exec(&line).is_some()) {
            nice += 1;
        }
    }

    println!("nice={}", nice);
}
