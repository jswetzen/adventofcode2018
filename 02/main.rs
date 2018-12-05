#![allow(dead_code)]
use std::collections::HashMap;

fn main() {
    let input = include_str!("input");

    p1(&input);
    p2(&input);
}

fn p1(input: &str) {
    let (mut two, mut three) = (0, 0);
    for line in input.lines() {
        let mut found = HashMap::new();
        for c in line.chars() {
            *found.entry(c).or_insert(0) += 1;
        }
        let (mut has_two, mut has_three) = (false, false);
        for (_c, count) in &found {
            if *count == 2 {
                has_two = true;
            }
            if *count == 3 {
                has_three = true;
            }
        }
        if has_two {
            two += 1;
        }
        if has_three {
            three += 1;
        }
    }
    println!("{}", two*three);
}

fn p2(input: &str) {
    let lines = String::from(input);
    let result = lines.lines().flat_map(|l1|
        lines.lines().map(move |l2| {
            if l1 == l2 {
                return String::new();
            } else {
                return matching(l1, l2);
            }
        })
    )
    .max_by(|x, y| x.len().cmp(&y.len()))
    .unwrap();

    println!("{}", result);
}

fn matching(l1: &str, l2: &str) -> String {
    l1.chars().zip(l2.chars())
        .filter(|(x, y)| x == y)
        .map(|(x, _y)| x)
        .collect::<String>()
}
