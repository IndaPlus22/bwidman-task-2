use std::{io, cmp::min};

fn main() {
    let stdin = io::stdin();

    let mut input = String::new();
    match stdin.read_line(&mut input) {
        Ok(..) => (),
        Err(..) => eprintln!("Could not read input"),
    }

    let dimensions = input.trim()
        .split(" ")
        .map(|value| value.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    
    let (r, k) = (dimensions[0], dimensions[1]);

    for y in 1..=r {
        for x in 1..=k {
            let dx = if x <= k / 2 { x } else { k - x + 1 };
            let dy = if y <= r / 2 { y } else { r - y + 1 };

            let distance = min(dx, dy);

            if distance <= 9 {
                print!("{distance}");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
