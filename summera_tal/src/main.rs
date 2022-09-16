use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    
    let mut lines = stdin
        .lock()
        .lines()
        .map(|line| line.ok().unwrap());
        
    let mut quantity: usize = lines
        .next().unwrap().trim()
        .parse().unwrap();

    if (quantity as f32) / 2.0 == (quantity / 2) as f32 { // Even
        quantity /= 2;
    } else { // Odd
        quantity = (quantity + 1) / 2;
    }

    let mut numbers = lines
        .next().unwrap()
        .split(" ")
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    numbers.sort();
    numbers.reverse();
    
    let mut sum: u32 = 0;
    for i in 0..quantity {
        sum += numbers[i];
    }
    println!("{}", sum);
}