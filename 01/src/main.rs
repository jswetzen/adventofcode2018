use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        lines.push(line.unwrap());
    }
    p1(&lines);
    p2(&lines);
}

fn p1(lines: &Vec<String>) {
    let mut count = 0;
    for num in lines {
        let val_s = &num.get(1..).unwrap();
        let val: i32 = val_s.parse().unwrap();
        if num.starts_with('+') {
            count += val;
        } else {
            count -= val;
        }
    }
    println!("{}", count);
}

fn p2(lines: &Vec<String>) {
    let mut count = 0;
    let mut freqs = HashSet::new();
    freqs.insert(count);
    for num in lines.iter().cycle() {
        let val_s = &num.get(1..).unwrap();
        let val: i32 = val_s.parse().unwrap();
        if num.starts_with('+') {
            count += val;
        } else {
            count -= val;
        }
        if freqs.contains(&count) {
            break;
        }
        freqs.insert(count);
    }
    println!("{}", count);
}
