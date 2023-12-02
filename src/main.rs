use std::fs::File;
use std::io::{prelude::*, BufReader};

fn day_01() {
    let file = File::open("input/input_01.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let shitty_numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
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
        sum += first * 10 + if last != -1 { last } else { first };
    }
    let mut sum2: i32 = 0;
    for line in lines.iter() {
        let mut numbas: Vec<(usize, usize)> = Vec::new();
        for (i, sn) in shitty_numbers.iter().enumerate() {
            if let Some(n) = line.find(sn) {
                numbas.push((i + 1, n));
            }
            if let Some(n) = line.rfind(sn) {
                numbas.push((i + 1, n));
            }
        }
        for (i, lsn) in less_shitty_numbers.iter().enumerate() {
            if let Some(n) = line.find(lsn) {
                numbas.push((i + 1, n));
            }
            if let Some(n) = line.rfind(lsn) {
                numbas.push((i + 1, n));
            }
        }
        numbas.sort_by(|(_, n1), (_, n2)| n1.cmp(n2));
        let tmp = (numbas.first().unwrap().0 * 10 + numbas.last().unwrap().0) as i32;
        sum2 += tmp;
    }
    println!("day01 {sum} {sum2}");
}

fn day_02() {
    let file = File::open("input/input_02.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let available = [12, 13, 14];
    let colors = ["red", "green", "blue"];
    let mut sum = 0;
    let mut sum2 = 0;
    for line in lines.iter() {
        let parts: Vec<&str> = line.split(' ').collect();
        let game_num = parts[1][..parts[1].len() - 1].parse::<i32>().unwrap();
        let mut passes = true;
        let mut num = 0;
        let mut min_set = [i32::MIN, i32::MIN, i32::MIN];
        for (i, &part) in parts[2..].iter().enumerate() {
            if i % 2 == 0 {
                num = part.parse::<i32>().unwrap();
            } else {
                let idx = if part.ends_with(',') || part.ends_with(';') {
                    colors
                        .iter()
                        .position(|x| *x == &part[..part.len() - 1])
                        .unwrap()
                } else {
                    colors.iter().position(|x| *x == part).unwrap()
                };
                if num > min_set[idx] {
                    min_set[idx] = num;
                }
                if available[idx] < num {
                    passes = false;
                }
            }
        }
        if passes {
            sum += game_num;
        }
        sum2 += min_set.iter().product::<i32>();
    }
    println!("day02 {sum} {sum2}");
}

fn main() {
    day_01();
    day_02();
}
