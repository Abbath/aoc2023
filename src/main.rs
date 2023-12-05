use rayon::prelude::*;
use std::collections::{HashMap, VecDeque};
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
    let mut cards: HashMap<i32, i32> = games.iter().map(|g| (g.id, 1)).collect();
    for f in found.iter() {
        *cards.entry(*f).or_insert(0) += 1;
    }
    while !found.is_empty() {
        let n = found.pop_front().unwrap();
        let mut counter = 1;
        for m in games[n as usize - 1].nums.iter() {
            if games[n as usize - 1].win.contains(m) {
                *cards.entry(games[n as usize - 1].id + counter).or_default() += 1;
                found.push_back(games[n as usize - 1].id + counter);
                counter += 1;
            }
        }
    }
    let sum2 = cards.iter().fold(0, |acc, (_, v)| acc + v);
    println!("day04 {sum} {sum2}");
}

fn day_05() {
    let file = File::open("input/input_05.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let mut seeds: Vec<u64> = Vec::new();
    type Span = (u64, u64, u64);
    enum MapType {
        None,
        Sts,
        Stf,
        Ftw,
        Wtl,
        Ltt,
        Tth,
        Htl,
    }
    let mut spans: Vec<Vec<Span>> = Vec::new();
    let mut spans2: Vec<(u64, u64)> = Vec::new();
    let mut sps: Vec<Span> = Vec::new();
    let mut flag = MapType::None;
    for line in lines.iter() {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("seeds:") {
            seeds = line
                .split(' ')
                .skip(1)
                .map(|s| s.parse::<u64>().unwrap())
                .collect();

            spans2 = seeds.chunks(2).map(|s| (s[0], s[1])).collect();
        }
        if line.starts_with("seed-to-soil") {
            flag = MapType::Sts;
            continue;
        }
        if line.starts_with("soil-to-fertilizer") {
            flag = MapType::Stf;
            spans.push(sps.clone());
            sps.clear();
            continue;
        }
        if line.starts_with("fertilizer-to-water") {
            flag = MapType::Ftw;
            spans.push(sps.clone());
            sps.clear();
            continue;
        }
        if line.starts_with("water-to-light") {
            flag = MapType::Wtl;
            spans.push(sps.clone());
            sps.clear();
            continue;
        }
        if line.starts_with("light-to-temperature") {
            flag = MapType::Ltt;
            spans.push(sps.clone());
            sps.clear();
            continue;
        }
        if line.starts_with("temperature-to-humidity") {
            flag = MapType::Tth;
            spans.push(sps.clone());
            sps.clear();
            continue;
        }
        if line.starts_with("humidity-to-location") {
            flag = MapType::Htl;
            spans.push(sps.clone());
            sps.clear();
            continue;
        }
        match flag {
            MapType::None => continue,
            _ => {
                let parts: Vec<u64> = line.split(' ').map(|s| s.parse::<u64>().unwrap()).collect();
                sps.push((parts[0], parts[1], parts[2]));
            }
        }
    }
    spans.push(sps.clone());
    sps.clear();
    let mut locations: Vec<u64> = Vec::new();
    fn check_span(val: u64, sps: Vec<Span>) -> u64 {
        for sp in sps {
            if (sp.1..sp.1 + sp.2).contains(&val) {
                return sp.0 + (val - sp.1);
            }
        }
        val
    }
    for seed in seeds.iter() {
        let mut cur = *seed;
        for sp in spans.iter() {
            cur = check_span(cur, sp.clone());
        }
        locations.push(cur);
    }
    let locations2 = spans2
        .par_iter()
        .map(|seedr| {
            let mut min_loc = u64::MAX;
            for seed in seedr.0..seedr.0 + seedr.1 {
                let mut cur = seed;
                for sp in spans.iter() {
                    cur = check_span(cur, sp.clone());
                }
                min_loc = min_loc.min(cur);
            }
            min_loc
        })
        .collect::<Vec<_>>();
    println!(
        "day05 {} {}",
        locations.iter().min().unwrap(),
        locations2.iter().min().unwrap()
    );
}

fn main() {
    day_01();
    day_02();
    day_03();
    day_04();
    day_05();
}
