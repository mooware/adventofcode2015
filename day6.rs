use std::io::stdin;
use std::io::BufRead;

struct Point(u32, u32);
struct Instruction { action : String, from : Point, to : Point }

fn parse_point(s : &str) -> Result<Point, String> {
    let parts = s.split(',').collect::<Vec<&str>>();

    if parts.len() != 2 {
        return Err("expected format 'num1,num2'".to_string());
    }

    let x = try!(parts[0].parse::<u32>().map_err(|e| e.to_string()));
    let y = try!(parts[1].parse::<u32>().map_err(|e| e.to_string()));

    Ok(Point(x, y))
}

fn parse_instruction(s : &str) -> Result<Instruction, String> {
    let mut parts = s.split(' ').collect::<Vec<&str>>();
    if parts.len() == 5 && parts[0] == "turn" {
        parts.remove(0);
    }

    if parts.len() != 4 {
        return Err("expected format '<action> x,y through x,y'".to_string());
    }

    let p1 = try!(parse_point(parts[1]));
    let p2 = try!(parse_point(parts[3]));

    Ok(Instruction{ action: parts[0].to_string(), from: p1, to: p2 })
}

fn main() {
    const SIZE : usize = 1000;

    let mut counts : Vec<bool> = Vec::new();
    counts.resize(SIZE * SIZE, false);

    let mut values : Vec<u32> = Vec::new();
    values.resize(SIZE * SIZE, 0);

    let mut count = 0;
    let mut value = 0;

    let stdin = stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let instruction = parse_instruction(&line).unwrap();

        let Point(x1, y1) = instruction.from;
        let Point(x2, y2) = instruction.to;
        let op : &str = &instruction.action;

        for y in y1..y2+1 {
            for x in x1..x2+1 {
                let index = (x + y * SIZE as u32) as usize;
                let oldc = counts[index];
                let oldv = values[index];

                let (newc, newv) = match op {
                    "on" => (true, oldv+1),
                    "off" => (false, oldv.checked_sub(1).unwrap_or(0)),
                    "toggle" => (!oldc, oldv+2),
                    _  => continue
                };

                counts[index] = newc;
                values[index] = newv;

                if !oldc && newc { count += 1; }
                else if oldc && !newc { count -= 1; }

                value -= oldv;
                value += newv;
            }
        }
    }

    println!("lights={}, brightness={}", count, value);
}
