extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();

    println!("check first 5 or 6 bytes? ");
    let input = stdin.lock().lines().next().unwrap().unwrap();

    let c = input.chars().nth(0).unwrap();
    let mask : u8 = if c == '6' { 0xff } else { 0xf0 };

    println!("enter secret key: ");
    let secret = stdin.lock().lines().next().unwrap().unwrap();

    let mut md5 = Md5::new();
    let mut result = [0u8; 16];
    let mut num = 1;

    loop {
        md5.input(secret.as_bytes());
        md5.input(num.to_string().as_bytes());
        
        md5.result(&mut result);

        let prefix = result[0] | result[1] | (result[2] & mask);
        if prefix == 0 {
            break;
        }

        md5.reset();
        num += 1;
    }

    println!("result={}", num);
}
