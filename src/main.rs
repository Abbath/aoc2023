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
    loop {
        let neighbors = gn(x, y, &lopp);
        for &(x2, y2) in neighbors.iter() {
            let a = at(&mat, x, y);
            let a2 = at(&mat, x2, y2);
            if x2 > x {
                match (a, a2) {
                    ('S', '-')
                    | ('S', 'J')
                    | ('S', '7')
                    | ('-', '-')
                    | ('-', 'J')
                    | ('-', '7')
                    | ('-', 'S')
                    | ('L', '-')
                    | ('L', 'J')
                    | ('L', '7')
                    | ('L', 'S')
                    | ('F', '-')
                    | ('F', 'J')
                    | ('F', '7')
                    | ('F', 'S') => {
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
                    _ => (),
                }
            }
            if x2 < x {
                match (a, a2) {
                    ('S', '-')
                    | ('S', 'F')
                    | ('S', 'L')
                    | ('-', '-')
                    | ('-', 'F')
                    | ('-', 'L')
                    | ('-', 'S')
                    | ('J', '-')
                    | ('J', 'F')
                    | ('J', 'L')
                    | ('J', 'S')
                    | ('7', '-')
                    | ('7', 'F')
                    | ('7', 'L')
                    | ('7', 'S') => {
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
                    _ => (),
                }
            }
            if y2 > y {
                match (a, a2) {
                    ('S', '|')
                    | ('S', 'J')
                    | ('S', 'L')
                    | ('|', '|')
                    | ('|', 'J')
                    | ('|', 'L')
                    | ('|', 'S')
                    | ('F', '|')
                    | ('F', 'J')
                    | ('F', 'L')
                    | ('F', 'S')
                    | ('7', '|')
                    | ('7', 'J')
                    | ('7', 'L')
                    | ('7', 'S') => {
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
                    _ => (),
                }
            }
            if y2 < y {
                match (a, a2) {
                    ('S', '|')
                    | ('S', 'F')
                    | ('S', '7')
                    | ('|', '|')
                    | ('|', 'F')
                    | ('|', '7')
                    | ('|', 'S')
                    | ('J', '|')
                    | ('J', 'F')
                    | ('J', '7')
                    | ('J', 'S')
                    | ('L', '|')
                    | ('L', 'F')
                    | ('L', '7')
                    | ('L', 'S') => {
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
                    _ => (),
                }
            }
        }
        if lopp.len() > 1 && lopp.first().unwrap() == lopp.last().unwrap() {
            break;
        }
    }
    (x, y) = *lopp.first().unwrap();
    mat[y * w + x] = match from_to {
        (0, -1, 0, 1) => '|',
        (-1, 0, 1, 0) => '-',
        (0, 1, 0, -1) => '|',
        (1, 0, -1, 0) => '-',
        (0, -1, 1, 0) => 'L',
        (1, 0, 0, -1) => 'L',
        (-1, 0, 0, 1) => '7',
        (0, 1, -1, 0) => '7',
        (0, -1, -1, 0) => 'J',
        (-1, 0, 0, -1) => 'J',
        (1, 0, 0, 1) => 'F',
        (0, 1, 1, 0) => 'F',
        _ => panic!("DUPA"),
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
}
