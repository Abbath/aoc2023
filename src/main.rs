use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::iter::zip;

fn day_01() {
    let file = File::open("input/input_01.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let shitty_numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let less_shitty_numbers = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let sum: i32 = lines
        .iter()
        .map(|line| {
            let first: i32 = line
                .chars()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap() as i32;
            let last: i32 = line
                .chars()
                .rfind(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap() as i32;
            first * 10 + if last != -1 { last } else { first }
        })
        .sum();
    let sum2: i32 = lines
        .iter()
        .map(|line| {
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
            (numbas.first().unwrap().0 * 10 + numbas.last().unwrap().0) as i32
        })
        .sum();
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

fn day_03() {
    let file = File::open("input/input_03.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let h = lines.len();
    let w = lines[0].len();
    let mat: Vec<char> = lines
        .iter()
        .flat_map(|s| s.chars().collect::<Vec<char>>())
        .collect();
    let at = |i: usize, j: usize| mat[i * w + j];
    let correct = |i: i32, j: i32| i >= 0 && i < h as i32 && j >= 0 && j < w as i32;
    let mut got_one = false;
    let mut sum = 0;
    let mut gears = HashMap::<(i32, i32), Vec<i32>>::new();
    let mut what_we_got = (' ', (0, 0));
    for i in 0..h as i32 {
        let mut numba = 0;
        for j in 0..w as i32 {
            let c = at(i as usize, j as usize);
            if c.is_ascii_digit() {
                for k in -1..=1 {
                    for l in -1..=1 {
                        if correct(i + k, j + l) {
                            let c2 = at((i + k) as usize, (j + l) as usize);
                            if !c2.is_ascii_digit() && c2 != '.' {
                                got_one = true;
                                if c2 == '*' {
                                    what_we_got = ('*', (i + k, j + l));
                                }
                            }
                        }
                    }
                }
                numba = numba * 10 + c.to_digit(10).unwrap();
                if got_one && j == w as i32 - 1 {
                    if what_we_got.0 == '*' {
                        gears.entry(what_we_got.1).or_default().push(numba as i32);
                        what_we_got = (' ', (0, 0));
                    }
                    got_one = false;
                    sum += numba;
                }
            } else {
                if got_one {
                    if what_we_got.0 == '*' {
                        gears.entry(what_we_got.1).or_default().push(numba as i32);
                        what_we_got = (' ', (0, 0));
                    }
                    got_one = false;
                    sum += numba;
                }
                numba = 0;
            }
        }
    }
    let sum2 = gears.iter().fold(
        0,
        |acc, (_, v)| {
            if v.len() == 2 {
                acc + v[0] * v[1]
            } else {
                acc
            }
        },
    );
    println!("day03 {sum} {sum2}");
}

fn day_04() {
    let file = File::open("input/input_04.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let mut sum = 0;
    struct Game {
        id: i32,
        win: Vec<i32>,
        nums: Vec<i32>,
        winnum: Option<i32>,
    }
    let mut games: Vec<Game> = Vec::new();
    let mut found: VecDeque<i32> = VecDeque::new();
    for line in lines.iter() {
        let parts = line
            .split(' ')
            .filter(|&s| !s.is_empty())
            .collect::<Vec<&str>>();
        let game_num = parts[1][..parts[1].len() - 1].parse::<i32>().unwrap();
        let mut found_bar = false;
        let mut worth = 0;
        let mut g = Game {
            id: game_num,
            win: Vec::new(),
            nums: Vec::new(),
            winnum: None,
        };
        let mut counter = 1;
        for part in parts.iter().skip(2) {
            if part == &"|" {
                found_bar = true;
                continue;
            }
            if let Ok(num) = part.parse::<i32>() {
                if !found_bar {
                    g.win.push(num);
                } else {
                    g.nums.push(num);
                    if g.win.contains(&num) {
                        found.push_back(game_num + counter);
                        counter += 1;
                        worth = if worth == 0 { 1 } else { worth * 2 };
                    }
                }
            }
        }
        games.push(g);
        sum += worth;
    }
    let mut sum2 = games.len() + found.len();
    while !found.is_empty() {
        let n = found.pop_front().unwrap();
        let g = &mut games[n as usize - 1];
        let new = if let Some(n) = g.winnum {
            n
        } else {
            let n = g
                .win
                .iter()
                .map(|m| if g.nums.contains(m) { 1 } else { 0 })
                .sum();
            g.winnum = Some(n);
            n
        };
        found.append(&mut VecDeque::from_iter(g.id + 1..=g.id + new));
        sum2 += new as usize;
    }
    println!("day04 {sum} {sum2}");
}

fn day_05() {
    let file = File::open("input/input_05.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let mut seeds: Vec<u64> = Vec::new();
    type Span = (u64, u64, u64);
    let mut spans: Vec<Vec<Span>> = Vec::new();
    let mut sps: Vec<Span> = Vec::new();
    let mut flag = false;
    let mut skip = false;
    for line in lines.iter() {
        if line.starts_with("seeds:") {
            seeds = line
                .split(' ')
                .skip(1)
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            continue;
        }
        if skip {
            skip = false;
            continue;
        }
        if line.is_empty() {
            if !flag {
                flag = true;
            } else {
                spans.push(sps.clone());
                sps.clear();
            }
            skip = true;
            continue;
        }
        let parts: Vec<u64> = line.split(' ').map(|s| s.parse::<u64>().unwrap()).collect();
        sps.push((parts[0], parts[1], parts[2]));
    }
    spans.push(sps.clone());
    sps.clear();
    let min_loc = seeds
        .iter()
        .map(|seed| {
            spans.iter().fold(*seed, |val, sps| {
                for sp in sps {
                    if (sp.1..sp.1 + sp.2).contains(&val) {
                        return sp.0 + (val - sp.1);
                    }
                }
                val
            })
        })
        .min()
        .unwrap();
    type Rng = (u64, u64);
    type RngSet = (Rng, Rng);
    let seed_ranges: Vec<Rng> = seeds.chunks(2).map(|x| (x[0], x[0] + x[1])).collect();
    let min_loc2 = spans
        .iter()
        .map(|s| {
            s.iter()
                .map(|(d, s, o)| ((*s, s + o), (*d, d + o)))
                .collect()
        })
        .fold(seed_ranges, |new_ranges, span_range: Vec<RngSet>| {
            new_ranges
                .iter()
                .flat_map(|in_range| {
                    let (mut resi, mut reso): (Vec<Rng>, Vec<Rng>) = span_range
                        .iter()
                        .flat_map(|rs| {
                            if in_range.1 < rs.0 .0 || rs.0 .1 < in_range.0 {
                                None
                            } else {
                                let input = (in_range.0.max(rs.0 .0), in_range.1.min(rs.0 .1));
                                Some((
                                    input,
                                    (rs.1 .0 + (input.0 - rs.0 .0), rs.1 .1 - (rs.0 .1 - input.1)),
                                ))
                            }
                        })
                        .unzip();
                    if reso.is_empty() {
                        reso.push(*in_range);
                        return reso;
                    }
                    resi.sort_by_key(|x| x.0);
                    if resi[0].0 > in_range.0 {
                        reso.push((in_range.0, resi[0].0 - 1))
                    }
                    for i in 1..resi.len() {
                        if resi[i].0 - resi[i - 1].1 > 1 {
                            reso.push((resi[i - 1].1 + 1, resi[i].0 - 1))
                        }
                    }
                    if resi[resi.len() - 1].1 < in_range.1 {
                        reso.push((resi[resi.len() - 1].1 + 1, in_range.1))
                    }
                    reso
                })
                .collect()
        })
        .iter()
        .min_by_key(|x| x.0)
        .unwrap()
        .0;
    println!("day05 {} {}", min_loc, min_loc2);
}

fn day_06() {
    let file = File::open("input/input_06.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let parser = |line: &str| {
        line.split(' ')
            .filter_map(|s| s.parse::<f64>().ok())
            .collect::<Vec<_>>()
    };
    let times = parser(&lines[0]);
    let distances = parser(&lines[1]);
    let solver = |t: f64, d: f64| {
        let ds = t * t - 4.0 * d;
        let x1 = (-t + ds.sqrt()) / -2.0;
        let x2 = (-t - ds.sqrt()) / -2.0;
        x2.floor() - x1.ceil() + 1.0
    };
    let product = zip(&times, &distances)
        .map(|(&t, &d)| solver(t, d))
        .product::<f64>();
    let joiner = |v: Vec<f64>| {
        v.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("")
            .parse::<f64>()
            .unwrap()
    };
    let time = joiner(times);
    let distance = joiner(distances);
    println!("day06 {product} {}", solver(time, distance));
}

fn main() {
    day_01();
    day_02();
    day_03();
    day_04();
    day_05();
    day_06();
}
