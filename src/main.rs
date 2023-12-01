use std::fs::File;
use std::io::{prelude::*, BufReader};

fn day_01() {
    let file = File::open("input/input_01.txt").unwrap();
    // let file = File::open("test_input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let shitty_numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let less_shitty_numbers = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut sum: i32 = 0;
    for line in lines.iter() {
        let mut first: i32 = -1;
        let mut last: i32 = -1;
        for ch in line.chars() {
            if first == -1 && ch.is_ascii_digit() {
                first = ch.to_digit(10).unwrap() as i32;
            } else if ch.is_ascii_digit() {
                last = ch.to_digit(10).unwrap() as i32;
            }
        }
        sum += first * 10 + if last != -1 {last} else {first};
    }
    let mut sum2: i32 = 0;
    for line in lines.iter() {
        let mut numbas: Vec<(usize, usize)> = Vec::new();
        for (i, sn) in shitty_numbers.iter().enumerate() {
            if let Some(n) = line.find(sn) {
                numbas.push((i+1, n));
            }
            if let Some(n) = line.rfind(sn) {
                numbas.push((i+1, n));
            }
        }
        for (i, lsn) in less_shitty_numbers.iter().enumerate() {
            if let Some(n) = line.find(lsn) {
                numbas.push((i+1, n));
            }
            if let Some(n) = line.rfind(lsn) {
                numbas.push((i+1, n));
            }
        }
        numbas.sort_by(|(_,n1), (_, n2)| n1.cmp(n2));
        let tmp = (numbas.first().unwrap().0 * 10 + numbas.last().unwrap().0) as i32;
        sum2 += tmp;
    }
    println!("day01 {sum} {sum2}");
}

fn main() {
    day_01();
}
