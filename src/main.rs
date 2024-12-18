use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::iter::zip;
use std::ops::RangeInclusive;
use std::usize;

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

fn day_07() {
    let file = File::open("input/input_07.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    #[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
    enum HandType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Hand {
        typ: HandType,
        cards: Vec<u32>,
        bid: u32,
    }
    let ctc = |c: char, part_two: bool| -> u32 {
        match c {
            '2'..='9' => c.to_digit(10).unwrap(),
            'T' => 10,
            'J' => {
                if part_two {
                    1
                } else {
                    11
                }
            }
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 0,
        }
    };
    let decide_type = |cs: &Vec<u32>| -> HandType {
        let mut hist: HashMap<u32, u32> = HashMap::new();
        for c in cs {
            *hist.entry(*c).or_default() += 1;
        }
        let vals = hist.values().copied().collect::<Vec<u32>>();
        if vals.contains(&5) {
            HandType::FiveOfAKind
        } else if vals.contains(&4) {
            HandType::FourOfAKind
        } else if vals.contains(&3) && vals.contains(&2) {
            HandType::FullHouse
        } else if vals.contains(&3) {
            HandType::ThreeOfAKind
        } else if vals.iter().filter(|x| *x == &2).count() == 2 {
            HandType::TwoPair
        } else if vals.contains(&2) {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    };
    let decide_type2 = |cs: &Vec<u32>| -> HandType {
        let mut hist: HashMap<u32, u32> = HashMap::new();
        for c in cs {
            *hist.entry(*c).or_default() += 1;
        }
        if !hist.contains_key(&1) {
            return decide_type(cs);
        }
        let jc = hist[&1];
        hist.remove(&1);
        let vals: Vec<_> = hist.values().copied().collect();
        let vc5 = vals.contains(&5);
        let vc4 = vals.contains(&4);
        let vc3 = vals.contains(&3);
        let vc2 = vals.contains(&2);
        let vc1 = vals.contains(&1);
        let vc22 = vals.iter().filter(|x| *x == &2).count() == 2;
        if vc5 || jc == 5 || vc4 && jc == 1 || vc3 && jc == 2 || vc2 && jc == 3 || vc1 && jc == 4 {
            HandType::FiveOfAKind
        } else if vc4 || vc3 && jc == 1 || vc2 && jc == 2 || vc1 && jc == 3 || jc == 4 {
            HandType::FourOfAKind
        } else if (jc == 2 || vc2) && vc3
            || vc2 && jc == 3
            || vc3 && vc1 && jc == 1
            || vc22 && jc == 1
        {
            HandType::FullHouse
        } else if vc3 || vc2 && jc == 1 || vc1 && jc == 2 || jc == 3 {
            HandType::ThreeOfAKind
        } else if vc22 || vc2 && jc == 2 || vc2 && vc1 && jc == 1 {
            HandType::TwoPair
        } else if vc2 || vc1 && jc == 1 || jc == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    };
    let (mut hands, mut hands2): (Vec<Hand>, Vec<Hand>) = lines
        .iter()
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<_>>();
            let v1: Vec<u32> = parts[0].chars().map(|c| ctc(c, false)).collect();
            let v2: Vec<u32> = parts[0].chars().map(|c| ctc(c, true)).collect();
            let bid = parts[1].parse::<u32>().unwrap();
            (
                Hand {
                    typ: decide_type(&v1),
                    cards: v1,
                    bid,
                },
                Hand {
                    typ: decide_type2(&v2),
                    cards: v2,
                    bid,
                },
            )
        })
        .unzip();
    hands.sort();
    hands2.sort();
    let compute = |hs: Vec<Hand>| {
        hs.iter()
            .enumerate()
            .map(|(i, x)| (i + 1) as u32 * x.bid)
            .sum::<u32>()
    };
    let sum = compute(hands);
    let sum2 = compute(hands2);
    println!("day07 {sum} {sum2}");
}

fn day_08() {
    let file = File::open("input/input_08.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let directions = lines[0].chars().collect::<Vec<_>>();
    let mapping = lines
        .iter()
        .skip(2)
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<_>>();
            let key = parts[0].to_string();
            let value = (
                parts[2].trim_matches('(').trim_matches(','),
                parts[3].trim_matches(')'),
            );
            (key, value)
        })
        .collect::<HashMap<_, _>>();
    let mut current = "AAA";
    let mut offset = 0;
    let mut counter = 0;
    while current != "ZZZ" {
        let dir = directions[offset];
        if dir == 'L' {
            current = mapping[current].0;
        } else {
            current = mapping[current].1;
        }
        offset = (offset + 1) % directions.len();
        counter += 1;
    }
    let mut currents = mapping
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    offset = 0;
    let mut counter2 = 0;
    let mut counters = vec![0; currents.len()];
    while !counters.iter().all(|x| *x > 0) {
        counter2 += 1;
        let dir = directions[offset];
        currents.iter_mut().enumerate().for_each(|(i, s)| {
            if dir == 'L' {
                *s = mapping[s].0.to_string();
            } else {
                *s = mapping[s].1.to_string();
            };
            if s.ends_with('Z') {
                counters[i] = counter2
            };
        });
        offset = (offset + 1) % directions.len();
    }
    use std::cmp::{max, min};
    fn gcd(a: usize, b: usize) -> usize {
        match ((a, b), (a & 1, b & 1)) {
            ((x, y), _) if x == y => y,
            ((0, x), _) | ((x, 0), _) => x,
            ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
            ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
            ((x, y), (1, 1)) => {
                let (x, y) = (min(x, y), max(x, y));
                gcd((y - x) >> 1, x)
            }
            _ => unreachable!(),
        }
    }
    fn lcm(a: usize, b: usize) -> usize {
        a * b / gcd(a, b)
    }
    let a2 = counters.iter().fold(1, |acc, c| lcm(acc, *c));
    println!("day08 {counter} {a2}");
}

fn day_09() {
    let file = File::open("input/input_09.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let (nexts, prevs): (Vec<i64>, Vec<i64>) = lines
        .iter()
        .map(|line| {
            let values = line
                .split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let mut diffs: Vec<Vec<i64>> = Vec::new();
            diffs.push(values);
            loop {
                let last_diff = diffs.last().unwrap();
                let diff = (1..last_diff.len())
                    .map(|i| last_diff[i] - last_diff[i - 1])
                    .collect::<Vec<_>>();
                if diff.iter().all(|x| *x == 0) {
                    break;
                }
                diffs.push(diff);
            }
            let mut next = 0;
            let mut prev = 0;
            for diff in diffs.iter().rev() {
                let last = diff.last().unwrap();
                let first = diff.first().unwrap();
                if next == 0 {
                    next = *last;
                } else {
                    next += last;
                }
                if prev == 0 {
                    prev = *first;
                } else {
                    prev = first - prev;
                }
            }
            (next, prev)
        })
        .unzip();
    let sum: i64 = nexts.iter().sum();
    let sum2: i64 = prevs.iter().sum();
    println!("day09 {sum} {sum2}");
}

fn day_10() {
    let file = File::open("input/input_10.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let mut mat: Vec<char> = Vec::new();
    let w = lines[0].len();
    let h = lines.len();
    mat.resize(w * h, ' ');
    mat = lines.iter().flat_map(|s| s.chars()).collect();
    let pos = mat.iter().position(|c| *c == 'S').unwrap();
    let mut x = pos % w;
    let mut y = pos / w;
    let mut lopp: Vec<(usize, usize)> = Vec::new();
    let gn = |x, y, c: &Vec<(usize, usize)>| {
        let mut res: Vec<(usize, usize)> = Vec::new();
        let nc = c
            .iter()
            .rev()
            .take(2)
            .copied()
            .collect::<Vec<(usize, usize)>>();
        if x > 0 && !nc.contains(&(x - 1, y)) {
            res.push((x - 1, y));
        }
        if y > 0 && !nc.contains(&(x, y - 1)) {
            res.push((x, y - 1));
        }
        if x < w - 1 && !nc.contains(&(x + 1, y)) {
            res.push((x + 1, y));
        }
        if y < h - 1 && !nc.contains(&(x, y + 1)) {
            res.push((x, y + 1));
        }
        res
    };
    let at = |mat: &Vec<char>, x, y| mat[y * w + x];
    let mut from_to = (0, 0, 0, 0);
    let left_bois = "S-LF";
    let right_bois = "S-J7";
    let top_bois = "S|JL";
    let bottom_bois = "S|7F";
    loop {
        let neighbors = gn(x, y, &lopp);
        for &(x2, y2) in neighbors.iter() {
            let a = at(&mat, x, y);
            let a2 = at(&mat, x2, y2);
            if x2 > x && left_bois.contains(a) && right_bois.contains(a2) {
                if a == 'S' {
                    from_to.0 = 1;
                }
                if a2 == 'S' {
                    from_to.2 = -1;
                }
                lopp.push((x, y));
                x = x2;
                break;
            }
            if x2 < x && right_bois.contains(a) && left_bois.contains(a2) {
                if a == 'S' {
                    from_to.0 = -1;
                }
                if a2 == 'S' {
                    from_to.2 = 1;
                }
                lopp.push((x, y));
                x = x2;
                break;
            }
            if y2 > y && bottom_bois.contains(a) && top_bois.contains(a2) {
                if a == 'S' {
                    from_to.1 = 1;
                }
                if a2 == 'S' {
                    from_to.3 = -1;
                }
                lopp.push((x, y));
                y = y2;
                break;
            }
            if y2 < y && top_bois.contains(a) && bottom_bois.contains(a2) {
                if a == 'S' {
                    from_to.1 = -1;
                }
                if a2 == 'S' {
                    from_to.3 = 1;
                }
                lopp.push((x, y));
                y = y2;
                break;
            }
        }
        if lopp.len() > 1 && lopp.first().unwrap() == lopp.last().unwrap() {
            break;
        }
    }
    (x, y) = *lopp.first().unwrap();
    mat[y * w + x] = match from_to {
        (0, -1, 0, 1) | (0, 1, 0, -1) => '|',
        (-1, 0, 1, 0) | (1, 0, -1, 0) => '-',
        (0, -1, 1, 0) | (1, 0, 0, -1) => 'L',
        (-1, 0, 0, 1) | (0, 1, -1, 0) => '7',
        (0, -1, -1, 0) | (-1, 0, 0, -1) => 'J',
        (1, 0, 0, 1) | (0, 1, 1, 0) => 'F',
        _ => panic!("Diagonals are not allowed"),
    };
    let mut sum = 0;
    for i in 0..h {
        let mut inside = false;
        for j in 0..w {
            if lopp.contains(&(j, i)) {
                if "|JL".contains(at(&mat, j, i)) {
                    inside = !inside;
                }
            } else if inside {
                sum += 1;
            }
        }
    }
    println!("day10 {} {}", lopp.len() / 2, sum);
}

fn day_11() {
    let file = File::open("input/input_11.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let mut empty_rows: HashSet<usize> = HashSet::new();
    let mut empty_cols: HashSet<usize> = (0..lines[0].len()).collect();
    for (row, line) in lines.iter().enumerate() {
        empty_rows.insert(row);
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((row, col));
                empty_rows.remove(&row);
                empty_cols.remove(&col);
            }
        }
    }
    let compute = |gs: &Vec<(usize, usize)>, offset: usize| {
        let mut sum = 0usize;
        for (n, g1) in gs.iter().enumerate() {
            for g2 in gs.iter().skip(n + 1) {
                let row_range = (g1.0.min(g2.0), g1.0.max(g2.0));
                let col_range = (g1.1.min(g2.1), g1.1.max(g2.1));
                let row_add = empty_rows
                    .iter()
                    .map(|r| {
                        if (row_range.0 + 1..row_range.1).contains(r) {
                            offset - 1
                        } else {
                            0
                        }
                    })
                    .sum::<usize>();
                let col_add = empty_cols
                    .iter()
                    .map(|c| {
                        if (col_range.0 + 1..col_range.1).contains(c) {
                            offset - 1
                        } else {
                            0
                        }
                    })
                    .sum::<usize>();
                sum +=
                    (row_range.1 - row_range.0) + (col_range.1 - col_range.0) + row_add + col_add;
            }
        }

        sum
    };
    println!(
        "day11 {} {}",
        compute(&galaxies, 2),
        compute(&galaxies, 1_000_000)
    );
}

fn day_12() {
    let file = File::open("input/input_12.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let compute = |s: &str, ns: &Vec<usize>| {
        let s2 = format!(".{}", s.trim_end_matches('.'))
            .chars()
            .collect::<Vec<_>>();
        let mut d = vec![0; s2.len() + 1];
        d[0] = 1;
        for (i, _) in s2.iter().take_while(|&&c| c != '#').enumerate() {
            d[i + 1] = 1;
        }
        *ns.iter()
            .fold(d, |d, &n| {
                let mut d2 = vec![0; s2.len() + 1];
                let mut counter = 0usize;
                for (i, &c) in s2.iter().enumerate() {
                    counter = if c != '.' { counter + 1 } else { 0 };
                    if c != '#' {
                        d2[i + 1] += d2[i];
                    }
                    if counter >= n && s2[i - n] != '#' {
                        d2[i + 1] += d[i - n];
                    }
                }
                d2
            })
            .last()
            .unwrap()
    };
    let (sum, sum2): (usize, usize) = lines
        .iter()
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<_>>();
            let numbers = parts[1]
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let string = parts[0];
            let sum = compute(string, &numbers);
            let string2 = [string; 5].join("?");
            let numbers2 = numbers
                .iter()
                .cycle()
                .take(5 * numbers.len())
                .copied()
                .collect::<Vec<_>>();
            let sum2 = compute(&string2, &numbers2);
            (sum, sum2)
        })
        .fold((0usize, 0usize), |(a, b), (c, d)| (a + c, b + d));
    println!("day12 {sum} {sum2}");
}

fn day_13() {
    let file = File::open("input/input_13.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let mut mats: Vec<(Vec<usize>, usize, usize)> = Vec::new();
    let mut mat: Vec<usize> = Vec::new();
    let mut counter = 0;
    let mut w = 0;
    for line in lines.iter() {
        if line.is_empty() {
            mats.push((mat.clone(), counter, w));
            mat.clear();
            counter = 0;
            continue;
        }
        if counter == 0 {
            w = line.len();
        }
        let mut v = line
            .chars()
            .map(|c| if c == '#' { 1 } else { 0usize })
            .collect::<Vec<_>>();
        mat.append(&mut v);
        counter += 1;
    }
    mats.push((mat, counter, w));
    fn vec_eq(v1: &Vec<&usize>, v2: &Vec<&usize>) -> usize {
        v1.len()
            .abs_diff(v1.iter().zip(v2).filter(|(&&a, &&b)| a == b).count())
    }
    fn vec_eq2(v1: &Vec<&usize>, v2: &Vec<&usize>, smudge: &mut bool) -> bool {
        let ve = vec_eq(v1, v2);
        if ve == 0 {
            true
        } else if ve == 1 && !*smudge {
            *smudge = true;
            true
        } else {
            false
        }
    }
    fn compute(mats: &[(Vec<usize>, usize, usize)], with_smudges: bool) -> i32 {
        let mut sum = 0;
        let mut counter;
        for (mat, rows, cols) in mats.iter() {
            let mut mirror_row = -1;
            let mut mirror_col = -1;
            for r in 1..*rows {
                let mut smudge_row = false;
                let row1 = mat
                    .iter()
                    .skip(cols * (r - 1))
                    .take(*cols)
                    .collect::<Vec<_>>();
                let row2 = mat.iter().skip(cols * r).take(*cols).collect::<Vec<_>>();
                let check = if with_smudges {
                    vec_eq2(&row1, &row2, &mut smudge_row)
                } else {
                    vec_eq(&row1, &row2) == 0
                };
                if mirror_row == -1 && check {
                    mirror_row = r as i32;
                    counter = 1;
                    while (r as i32 - 1 - counter as i32) >= 0 && r + counter < *rows {
                        let a = r - counter - 1;
                        let b = r + counter;
                        let row1 = mat.iter().skip(cols * a).take(*cols).collect::<Vec<_>>();
                        let row2 = mat.iter().skip(cols * b).take(*cols).collect::<Vec<_>>();
                        let check = if with_smudges {
                            vec_eq2(&row1, &row2, &mut smudge_row)
                        } else {
                            vec_eq(&row1, &row2) == 0
                        };
                        if !check {
                            mirror_row = -1;
                            break;
                        }
                        counter += 1;
                    }
                    if with_smudges && !smudge_row {
                        mirror_row = -1;
                    }
                }
            }
            for c in 1..*cols {
                let mut smudge_col = false;
                let col1 = mat.iter().skip(c - 1).step_by(*cols).collect::<Vec<_>>();
                let col2 = mat.iter().skip(c).step_by(*cols).collect::<Vec<_>>();
                let check = if with_smudges {
                    vec_eq2(&col1, &col2, &mut smudge_col)
                } else {
                    vec_eq(&col1, &col2) == 0
                };
                if mirror_col == -1 && check {
                    mirror_col = c as i32;
                    counter = 1;
                    while (c as i32 - 1 - counter as i32) >= 0 && c + counter < *cols {
                        let a = c - counter - 1;
                        let b = c + counter;
                        let col1 = mat.iter().skip(a).step_by(*cols).collect::<Vec<_>>();
                        let col2 = mat.iter().skip(b).step_by(*cols).collect::<Vec<_>>();
                        let check = if with_smudges {
                            vec_eq2(&col1, &col2, &mut smudge_col)
                        } else {
                            vec_eq(&col1, &col2) == 0
                        };
                        if !check {
                            mirror_col = -1;
                            break;
                        }
                        counter += 1;
                    }
                    if with_smudges && !smudge_col {
                        mirror_col = -1;
                    }
                }
            }
            if mirror_row > 0 {
                sum += 100 * mirror_row
            }
            if mirror_col > 0 {
                sum += mirror_col;
            }
        }
        sum
    }
    println!("day13 {} {}", compute(&mats, false), compute(&mats, true));
}

fn day_14() {
    let file = File::open("input/input_14.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let rows = lines.len();
    let cols = lines[0].len();
    let mat = lines
        .iter()
        .flat_map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    #[derive(Copy, Clone)]
    enum Dir {
        North,
        West,
        South,
        East,
    }
    fn tilt(mat: &[char], rows: usize, cols: usize, dir: Dir) -> Vec<char> {
        let mut res = mat.to_owned();
        let mut pos = vec![
            match dir {
                Dir::North | Dir::West => 0usize,
                Dir::South => rows - 1,
                Dir::East => cols - 1,
            };
            match dir {
                Dir::North | Dir::South => cols,
                Dir::West | Dir::East => rows,
            }
        ];
        let outer_range = match dir {
            Dir::North => (0, rows - 1),
            Dir::South => (rows - 1, 0),
            Dir::West => (0, cols - 1),
            Dir::East => (cols - 1, 0),
        };
        let inner_range = match dir {
            Dir::North => (0, cols - 1),
            Dir::South => (0, cols - 1),
            Dir::West => (0, rows - 1),
            Dir::East => (0, rows - 1),
        };
        let idx = |r: usize, c: usize, dir: Dir| match dir {
            Dir::North | Dir::South => r * cols + c,
            Dir::West | Dir::East => c * cols + r,
        };
        let advance_pos = |pos: usize, dir: Dir| match dir {
            Dir::North | Dir::West => pos + 1,
            Dir::South | Dir::East => pos - 1,
        };
        let compare_pos = |val: usize, pos: usize, dir: Dir| match dir {
            Dir::North | Dir::West => val > pos,
            Dir::South | Dir::East => val < pos,
        };
        fn mkiter<'a, T: 'a>(start: T, end: T) -> Box<dyn Iterator<Item = T> + 'a>
        where
            T: PartialOrd,
            RangeInclusive<T>: Iterator<Item = T> + DoubleEndedIterator,
        {
            if start > end {
                Box::new((end..=start).rev())
            } else {
                Box::new(start..=end)
            }
        }
        for r in mkiter(outer_range.0, outer_range.1) {
            for c in mkiter(inner_range.0, inner_range.1) {
                match mat[idx(r, c, dir)] {
                    '.' => (),
                    'O' => {
                        if compare_pos(r, pos[c], dir) {
                            res[idx(pos[c], c, dir)] = 'O';
                            res[idx(r, c, dir)] = '.';
                            pos[c] = advance_pos(pos[c], dir);
                        } else if r == pos[c] {
                            pos[c] = advance_pos(pos[c], dir);
                        }
                    }
                    '#' => pos[c] = advance_pos(r, dir),
                    _ => panic!("Wrong character"),
                }
            }
        }
        res
    }
    fn compute_load(mat: &[char], rows: usize, cols: usize) -> usize {
        let idx = |r, c| r * cols + c;
        let mut sum = 0;
        for r in 0..rows {
            for c in 0..cols {
                if mat[idx(r, c)] == 'O' {
                    sum += rows - r;
                }
            }
        }
        sum
    }
    fn cycle(mat: &[char], rows: usize, cols: usize) -> Vec<char> {
        let a = tilt(mat, rows, cols, Dir::North);
        let b = tilt(&a, rows, cols, Dir::West);
        let c = tilt(&b, rows, cols, Dir::South);
        tilt(&c, rows, cols, Dir::East)
    }
    let new_mat = tilt(&mat, rows, cols, Dir::North);
    fn vec_eq(v1: &Vec<char>, v2: &Vec<char>) -> bool {
        v1.iter().zip(v2).filter(|(&a, &b)| a == b).count() == v1.len()
    }
    let mut v: Vec<char> = mat.clone();
    let mut mats: Vec<(Vec<char>, usize, usize)> = Vec::new();
    mats.push((v.clone(), compute_load(&v, rows, cols), 0));
    let big = 1_000_000_000;
    let mut load = 0;
    for i in 0..big {
        v = cycle(&v, rows, cols);
        let val = compute_load(&v, rows, cols);
        if let Some((_, _, n)) = mats.iter().find(|(x, l, _)| vec_eq(&v, x) && *l == val) {
            let diff = (i + 1) - n;
            load = mats[n + (big - n) % diff].1;
            break;
        }
        mats.push((v.clone(), val, i + 1));
    }
    println!("day14 {} {}", compute_load(&new_mat, rows, cols), load);
}

fn day_15() {
    let file = File::open("input/input_15.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let line = lines.join("");
    let parts = line.split(',').collect::<Vec<_>>();
    let mut boxes: Vec<VecDeque<(String, u32)>> = vec![VecDeque::new(); 256];
    let hash = |s: &String| {
        s.chars()
            .fold(0usize, |acc, c| (acc + c as usize) * 17 % 256)
    };
    let sum = parts
        .iter()
        .map(|part| {
            if part.ends_with('-') {
                let label = part.strip_suffix('-').unwrap().to_string();
                let h = hash(&label);
                if let Some(pos) = boxes[h].iter().position(|(s, _)| **s == label) {
                    boxes[h].remove(pos);
                }
            } else {
                let label = part[..part.len() - 2].to_string();
                let h = hash(&label);
                let power = part.chars().last().unwrap().to_digit(10).unwrap();
                if let Some(pos) = boxes[h].iter().position(|(s, _)| **s == label) {
                    boxes[h][pos].1 = power;
                } else {
                    boxes[h].push_back((label.clone(), power));
                }
            }
            hash(&part.to_string())
        })
        .sum::<usize>();
    let sum2 = boxes
        .iter()
        .enumerate()
        .filter_map(|(i, item)| {
            if !item.is_empty() {
                Some(
                    item.iter()
                        .enumerate()
                        .map(|(j, (_, n))| (i + 1) * (j + 1) * *n as usize)
                        .sum::<usize>(),
                )
            } else {
                None
            }
        })
        .sum::<usize>();
    println!("day15 {sum} {sum2}");
}

fn day_16() {
    let file = File::open("input/input_16.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let rows = lines.len();
    let cols = lines[0].len();
    let mat: Vec<char> = lines
        .iter()
        .flat_map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Beam {
        r: i32,
        c: i32,
        dx: i32,
        dy: i32,
        alive: bool,
    }
    let beam = Beam {
        r: 0,
        c: 0,
        dx: 1,
        dy: 0,
        alive: true,
    };
    fn compute(beam: &Beam, mat: &Vec<char>, rows: usize, cols: usize) -> usize {
        let idx = |r: i32, c: i32| -> usize { r as usize * cols + c as usize };
        let mut energized: Vec<usize> = vec![0usize; mat.len()];
        let mut beams: VecDeque<Beam> = VecDeque::new();
        beams.push_back(*beam);
        let mut all_beams: HashSet<Beam> = HashSet::new();
        let mut insert_beam = |b, beams: &mut VecDeque<Beam>| {
            if !all_beams.contains(&b) {
                beams.push_back(b);
                all_beams.insert(b);
            }
        };
        while !beams.is_empty() {
            let mut beam = beams.pop_front().unwrap();
            while beam.alive {
                if beam.r < 0 || beam.c < 0 || beam.r >= rows as i32 || beam.c >= cols as i32 {
                    beam.alive = false;
                    break;
                }
                energized[idx(beam.r, beam.c)] = 1;
                if "\\/".contains(mat[idx(beam.r, beam.c)]) {
                    match (mat[idx(beam.r, beam.c)], beam.dx, beam.dy) {
                        ('/', 1, 0) | ('/', -1, 0) => {
                            beam.dy = -beam.dx;
                            beam.dx = 0
                        }
                        ('/', 0, 1) | ('/', 0, -1) => {
                            beam.dx = -beam.dy;
                            beam.dy = 0
                        }
                        ('\\', 1, 0) | ('\\', -1, 0) => {
                            beam.dy = beam.dx;
                            beam.dx = 0
                        }
                        ('\\', 0, 1) | ('\\', 0, -1) => {
                            beam.dx = beam.dy;
                            beam.dy = 0
                        }
                        _ => (),
                    }
                } else if "-|".contains(mat[idx(beam.r, beam.c)]) {
                    match (mat[idx(beam.r, beam.c)], beam.dx, beam.dy) {
                        ('|', 1, 0) | ('|', -1, 0) => {
                            beam.alive = false;
                            insert_beam(
                                Beam {
                                    r: beam.r - 1,
                                    c: beam.c,
                                    dx: 0,
                                    dy: -1,
                                    alive: true,
                                },
                                &mut beams,
                            );
                            insert_beam(
                                Beam {
                                    r: beam.r + 1,
                                    c: beam.c,
                                    dx: 0,
                                    dy: 1,
                                    alive: true,
                                },
                                &mut beams,
                            );
                            break;
                        }
                        ('-', 0, 1) | ('-', 0, -1) => {
                            beam.alive = false;
                            insert_beam(
                                Beam {
                                    r: beam.r,
                                    c: beam.c - 1,
                                    dx: -1,
                                    dy: 0,
                                    alive: true,
                                },
                                &mut beams,
                            );
                            insert_beam(
                                Beam {
                                    r: beam.r,
                                    c: beam.c + 1,
                                    dx: 1,
                                    dy: 0,
                                    alive: true,
                                },
                                &mut beams,
                            );
                            break;
                        }
                        _ => (),
                    }
                }
                beam.r += beam.dy;
                beam.c += beam.dx;
            }
        }
        energized.iter().sum::<usize>()
    }
    let e1 = (0..cols)
        .map(|c| {
            let beam = Beam {
                r: 0,
                c: c as i32,
                dx: 0,
                dy: 1,
                alive: true,
            };
            compute(&beam, &mat, rows, cols)
        })
        .max()
        .unwrap();
    let e2 = (0..cols)
        .map(|c| {
            let beam = Beam {
                r: rows as i32 - 1,
                c: c as i32,
                dx: 0,
                dy: -1,
                alive: true,
            };
            compute(&beam, &mat, rows, cols)
        })
        .max()
        .unwrap();
    let e3 = (0..rows)
        .map(|r| {
            let beam = Beam {
                r: r as i32,
                c: 0,
                dx: 1,
                dy: 0,
                alive: true,
            };
            compute(&beam, &mat, rows, cols)
        })
        .max()
        .unwrap();
    let e4 = (0..rows)
        .map(|r| {
            let beam = Beam {
                r: r as i32,
                c: cols as i32 - 1,
                dx: -1,
                dy: 0,
                alive: true,
            };
            compute(&beam, &mat, rows, cols)
        })
        .max()
        .unwrap();
    println!(
        "day16 {} {}",
        compute(&beam, &mat, rows, cols),
        &[e1, e2, e3, e4].iter().max().unwrap()
    );
}

fn day_17() {
    let file = File::open("test_input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let rows = lines.len();
    let cols = lines[0].len();
    let mat: Vec<u64> = lines
        .iter()
        .flat_map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    enum Dir {
        No,
        North,
        West,
        South,
        East,
    }
    // impl Dir {
    //     pub fn invert(self) -> Self {
    //         match self {
    //             Dir::North => Dir::South,
    //             Dir::South => Dir::North,
    //             Dir::East => Dir::West,
    //             Dir::West => Dir::East,
    //             Dir::No => Dir::No
    //         }
    //     }
    // }
    let neighbours = |field: &Vec<u64>, idx: usize, h: usize, w: usize, dir: Dir| -> Vec<(usize, usize, Dir)> {
        let mut res = Vec::new();
        let i = idx / w;
        let j = idx % w;
        if i > 0 && dir != Dir::North {
            res.push((
                (i - 1) * w + j,
                field[(i - 1) * w + j] as usize,
                Dir::South
            ));
        } else {
            println!("{i} {j} Can't north");
        }
        if j > 0 && dir != Dir::West {
            res.push((
                i * w + j - 1,
                field[i * w + j - 1] as usize,
                Dir::East
            ));
        } else {
            println!("{i} {j} Can't west");
        }
        if i < h - 1 && dir != Dir::South {
            res.push((
                (i + 1) * w + j,
                field[(i + 1) * w + j] as usize,
                Dir::North
            ));
        } else {
            println!("{i} {j} Can't south");
        }
        if j < w - 1 && dir != Dir::East {
            res.push((
                i * w + j + 1,
                field[i * w + j + 1] as usize,
                Dir::West
            ));
        } else {
            println!("{i} {j} Can't east");
        }
        res
    };
    fn reconstruct_path(came_from: &HashMap<usize, (usize, Dir)>, current: usize) -> VecDeque<usize> {
        let mut total_path = VecDeque::from(vec![current]);
        let mut curr = current;
        while came_from.contains_key(&curr) {
            curr = came_from[&curr].0;
            total_path.push_front(curr);
        }
        total_path
    }
    let astar =
        |field: &Vec<u64>, h: usize, w: usize, start: (usize, usize), finish: (usize, usize)| {
            let hh = |u: usize| {
                field[u]
            };
            let rows = h * w;
            let mut open_set = BinaryHeap::new();
            let mut came_from: HashMap<usize, (usize, Dir)> = HashMap::new();
            open_set.push((Reverse(0), (0, vec![(0,Dir::No)])));
            let mut g_score = vec![rows * 100; rows];
            g_score[start.0 * w + start.1] = 0;
            let mut f_score = vec![rows * 100; rows];
            f_score[start.0 * w + start.1] = 0;
            let mut res = 0;
            while let Some((Reverse(_), (idx, dirs))) = open_set.pop() {
                if idx == finish.0 * w + finish.1 {
                    res = idx;
                    break;
                }
                println!("{dirs:?}");
                let last_three = if dirs.len() >= 3 {
                    dirs[dirs.len()-3..].iter().map(|x|x.1).collect()
                } else {
                    vec![]
                };
                let last = *dirs.last().unwrap();
                for (k, v, dir) in neighbours(field, idx, h, w, last.1) {
                    let new_score = g_score[idx] + v as usize;
                    let same = last_three.len() == 3 && (last_three[0] == last_three[1] || last_three[0] == Dir::No) && last_three[1] == last_three[2] && last_three[2] == dir;
                    if new_score < g_score[k] && !same {
                        came_from.insert(k, (idx, last.1));
                        g_score[k] = new_score;
                        f_score[k] = new_score + hh(k) as usize;
                        let mut new_dirs = dirs.clone();
                        new_dirs.push((k,dir));
                        open_set.push((Reverse(f_score[k]), (k, new_dirs)));
                    }
                }
            }
            let path = reconstruct_path(&came_from, res);
            println!("{path:?}");
            for r in 0..h {
                for c in 0..w {
                    if path.contains(&(r * w + c)) {
                        print!("X");
                    } else {
                        print!("{}", field[r * w + c]);
                    }
                }
                println!();
            }
            let l = path.iter().map(|x| field[*x] as usize).sum();
            if l == 0 {
                usize::MAX
            } else {
                l
            }
        };
    println!("{}", astar(&mat, rows, cols, (0, 0), (rows-1, cols-1)));
}

fn day_18() {
    let file = File::open("test_input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    #[derive(Debug)]
    enum Dir {
        Right,
        Left,
        Up,
        Down
    }
    let dl = lines.iter().map(|line| {
        let parts = line.split(' ').collect::<Vec<_>>();
        let dir = match parts[0] {
            "R" => Dir::Right,
            "L" => Dir::Left,
            "U" => Dir::Up,
            "D" => Dir::Down,
            _ => panic!("Wrong letter")
        };
        let l = parts[1].parse::<i64>().unwrap();
        (dir, l)
    }).collect::<Vec<_>>();
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;
    let mut x = 0i64;
    let mut y = 0i64;
    for (d, l) in dl.iter() {
        match d {
            Dir::Right => x += l,
            Dir::Left => x -= l,
            Dir::Up => y -= l,
            Dir::Down => y += l,
        }
        min_x = x.min(min_x);
        min_y = y.min(min_y);
        max_x = x.max(max_x);
        max_y = y.max(max_y);
    }
    let rows = (max_y - min_y) as usize + 1;
    let cols = (max_x - min_x) as usize + 1;
    let mut mat = vec!['.'; rows * cols];
    let idx = |r: usize, c: usize| r * cols + c;
    let mut x = (0 - min_x) as usize;
    let mut y = (0 - min_y) as usize;
    for (d, l) in dl.iter() {
        (0..*l).for_each(|_| {
            mat[idx(y,x)] = '#';
            match d {
                Dir::Right => {
                    x += 1;
                },
                Dir::Left => {
                    x -= 1;
                },
                Dir::Up => {
                    y -= 1;
                },
                Dir::Down => {
                    y += 1;
                },
            }
        });
    }
    let mut sum = 0;
    let mut inside = false;
    for r in 0..rows {
        for c in 0..cols {
            if mat[idx(r,c)] == '#' {
                if r > 0 && mat[idx(r-1,c)] == '#' {
                    inside = !inside;
                }
                sum += 1;
            }
            if inside && mat[idx(r,c)] != '#' {
                sum += 1;
            }
        }
    }
    // for r in 0..rows {
    //     for c in 0..cols {
    //         print!("{}", mat[idx(r,c)]);
    //     }
    //     println!();
    // }
    let dl2 = lines.iter().map(|line| {
        let parts = line.split(' ').collect::<Vec<_>>();
        let num = usize::from_str_radix(&parts[2][2..parts[2].len()-1], 16).unwrap();
        let dir = match num % 16 {
            0 => Dir::Right,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Up,
            _ => panic!("Wrong number")
        };
        let l = num / 16;
        (dir, l)
    }).collect::<Vec<_>>();
    println!("{dl2:?}");
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;
    let mut x = 0i64;
    let mut y = 0i64;
    for (d, l) in dl2.iter() {
        match d {
            Dir::Right => x += *l as i64,
            Dir::Left => x -= *l as i64,
            Dir::Up => y -= *l as i64,
            Dir::Down => y += *l as i64,
        }
        min_x = x.min(min_x);
        min_y = y.min(min_y);
        max_x = x.max(max_x);
        max_y = y.max(max_y);
    }
    let rows = (max_y - min_y) as usize + 1;
    // let cols = (max_x - min_x) as usize + 1;
    let mut x = (0 - min_x) as usize;
    let mut y = (0 - min_y) as usize;
    let mut mp: HashMap<usize, Vec<usize>> = HashMap::new();
    for (d, l) in dl2.iter() {
        (0..*l).for_each(|_| {
            mp.entry(y).or_default().push(x);
            match d {
                Dir::Right => {
                    x += 1;
                },
                Dir::Left => {
                    x -= 1;
                },
                Dir::Up => {
                    y -= 1;
                },
                Dir::Down => {
                    y += 1;
                },
            }
        });
    }
    let mut sum2 = 0;
    for r in 0..rows {
        let mut cs: Vec<usize> = mp[&r].clone();
        cs.sort();
        let mut inside = false;
        for i in 0..cs.len() {
            // sum2 += 1;
            let c = cs[i];
            if r > 0 && mp[&(r-1)].contains(&c) {
                inside = !inside;
            }
            if inside {
                sum2 += cs[i+1] - c;
                sum2 += 1;
            }
        }
        // for c in 0..cols {
        //     if mat[idx(r,c)] == '#' {
        //         if r > 0 && mat[idx(r-1,c)] == '#' {
        //             inside = !inside;
        //         }
        //         sum += 1;
        //     }
        //     if inside && mat[idx(r,c)] != '#' {
        //         sum += 1;
        //     }
        // }
    }
    println!("day18 {sum} {sum2:?}");
}

fn day_19() {
    let file = File::open("input/input_19.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let mut two = false;
    #[derive(Debug, Clone)]
    enum CondType {
        Less,
        More
    }
    #[derive(Debug, Clone)]
    enum Rule {
        Reject,
        Accept,
        Cond(String, CondType, usize, Box<Rule>),
        Jump(String)
    }
    #[derive(Debug)]
    struct Part {
        x: usize,
        m: usize,
        a: usize,
        s: usize
    }
    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();
    for line in lines.iter() {
        if line.is_empty() {
            two = true;
            continue;
        }
        if !two {
            let (nm, rest) = line.split_once('{').unwrap();
            let parts = rest[..rest.len()-1].split(',').collect::<Vec<_>>();
            let rules = parts.iter().map(|&part| {
                match part {
                    "R" => Rule::Reject,
                    "A" => Rule::Accept,
                    _ => {
                        if part.contains(':') {
                            let parts2 = part.split(|c| c == '<' || c == '>' || c == ':').collect::<Vec<_>>();
                            let ct = if part.contains('<') {
                                CondType::Less
                            } else {
                                CondType::More
                            };
                            let pn = parts2[0].to_string();
                            let val = parts2[1].parse::<usize>().unwrap();
                            let rule = match parts2[2] {
                                "R" => Rule::Reject,
                                "A" => Rule::Accept,
                                _ => Rule::Jump(parts2[2].to_string())
                            };
                            Rule::Cond(pn, ct, val, Box::new(rule))
                        } else {
                            Rule::Jump(part.to_string())
                        }
                    }
                }
            }).collect::<Vec<_>>();
            workflows.insert(nm.to_string(), rules);
        } else {
            let line = line[1..line.len()-1].to_string();
            let props = line.split(',').map(|part| {
                let (n, v) = part.split_once('=').unwrap();
                (n, v.parse::<usize>().unwrap())
            }).collect::<HashMap<_, _>>();
            parts.push(Part{x: props["x"], m: props["m"], a: props["a"], s: props["s"]});
        }
    }
    let sum = parts.iter().map(|part| {
        let mut state: String = "in".to_string();
        'outer: loop {
            let rules = workflows[&state].clone();
            for rule in rules.iter() {
                match rule {
                    Rule::Reject => break 'outer 0,
                    Rule::Accept => break 'outer (part.x + part.m + part.a + part.s),
                    Rule::Jump(sname) => {
                        state = sname.clone();
                        continue 'outer;
                    },
                    Rule::Cond(pn, ct, val, r) => {
                        let v = match pn.as_str() {
                            "x" => part.x,
                            "m" => part.m,
                            "a" => part.a,
                            "s" => part.s,
                            _ => panic!("Wrong property")
                        };
                        let c = match ct {
                            CondType::Less => v < *val,
                            CondType::More => v > *val
                        };
                        if c {
                            let ru = *r.clone();
                            match ru {
                                Rule::Reject => break 'outer 0,
                                Rule::Accept => break 'outer (part.x + part.m + part.a + part.s),
                                Rule::Jump(sname) => {
                                    state = sname.clone();
                                    continue 'outer;
                                },
                                _ => panic!("Wrong rule")
                            }
                        }
                    }
                }
            }
        }
    }).sum::<usize>();
    println!("day19 {sum:?}");
}

fn main() {
    day_01();
    day_02();
    day_03();
    day_04();
    day_05();
    day_06();
    day_07();
    day_08();
    day_09();
    day_10();
    day_11();
    day_12();
    day_13();
    day_14();
    day_15();
    day_16();
    // day_17();
    // day_18();
    day_19();
}
