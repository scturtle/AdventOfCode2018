use itertools::{iproduct, Itertools};
use std::collections::{HashSet, VecDeque};

enum Route {
    Plain(Vec<u8>),
    Seq(Vec<Route>),
    Brh(Vec<Route>),
}
use Route::*;

fn find_match_paren(ss: &[u8], s: usize) -> usize {
    assert_eq!(ss[s], b'(');
    let mut i = s + 1;
    let mut p = 1;
    loop {
        match ss[i] {
            b'(' => p += 1,
            b')' => p -= 1,
            _ => {}
        }
        if p == 0 {
            return i;
        }
        i += 1;
    }
}

fn find_next_branch(ss: &[u8], s: usize, e: usize) -> usize {
    let mut i = s;
    let mut p = 0;
    while i < e {
        match ss[i] {
            b'(' => p += 1,
            b')' => p -= 1,
            _ => {}
        }
        if p == 0 && ss[i] == b'|' {
            return i;
        }
        i += 1;
    }
    e
}

fn parse(ss: &[u8], s: usize, t: usize) -> Route {
    let mut brhs = vec![];
    let mut i = s;
    while i < t {
        let ni = find_next_branch(ss, i, t);
        brhs.push(parse_non_brh(ss, i, ni));
        i = ni + 1;
    }
    Brh(brhs)
}

fn parse_non_brh(ss: &[u8], s: usize, t: usize) -> Route {
    let mut seqs: Vec<Route> = vec![];
    let mut i = s;
    let mut plain: Vec<u8> = vec![];
    while i < t {
        match ss[i] {
            b'(' => {
                if !plain.is_empty() {
                    let mut tmp = vec![];
                    std::mem::swap(&mut plain, &mut tmp);
                    seqs.push(Plain(tmp));
                }
                let pt = find_match_paren(ss, i);
                seqs.push(parse(ss, i + 1, pt));
                i = pt + 1;
            }
            c => {
                plain.push(c);
                i += 1;
            }
        }
    }
    if !plain.is_empty() {
        seqs.push(Plain(plain));
    }
    Seq(seqs)
}

fn simplify(r: Route) -> Route {
    match r {
        Brh(v) => match v.len() {
            0 => Plain(vec![]),
            1 => simplify(v.into_iter().nth(0).unwrap()),
            _ => Brh(v.into_iter().map(simplify).collect_vec()),
        },
        Seq(v) => match v.len() {
            0 => Plain(vec![]),
            1 => simplify(v.into_iter().nth(0).unwrap()),
            _ => Seq(v.into_iter().map(simplify).collect_vec()),
        },
        r => r,
    }
}

fn step(m: &mut [Vec<u8>], x: usize, y: usize, c: u8) -> (usize, usize) {
    m[y][x] = b'.';
    m[y - 1][x - 1] = b'#';
    m[y + 1][x - 1] = b'#';
    m[y - 1][x + 1] = b'#';
    m[y + 1][x + 1] = b'#';
    let (y, x) = match c {
        b'N' => {
            m[y - 1][x] = b'-';
            (y - 2, x)
        }
        b'S' => {
            m[y + 1][x] = b'-';
            (y + 2, x)
        }
        b'W' => {
            m[y][x - 1] = b'|';
            (y, x - 2)
        }
        b'E' => {
            m[y][x + 1] = b'|';
            (y, x + 2)
        }
        _ => unreachable!(),
    };
    m[y][x] = b'.';
    m[y - 1][x - 1] = b'#';
    m[y + 1][x - 1] = b'#';
    m[y - 1][x + 1] = b'#';
    m[y + 1][x + 1] = b'#';
    (x, y)
}

fn walk(m: &mut [Vec<u8>], x: usize, y: usize, r: &Route) -> Vec<(usize, usize)> {
    match r {
        Plain(cs) => {
            let mut x = x;
            let mut y = y;
            for c in cs {
                let (nx, ny) = step(m, x, y, *c);
                x = nx;
                y = ny;
            }
            vec![(x, y)]
        }
        Seq(rs) => {
            let mut xys = vec![(x, y)];
            for r in rs {
                let mut next_xys = xys
                    .iter()
                    .flat_map(|&(x, y)| walk(m, x, y, r))
                    .collect_vec();
                next_xys.sort();
                next_xys.dedup();
                xys = next_xys;
            }
            xys
        }
        Brh(rs) => {
            let mut all_next_xys = vec![];
            for r in rs {
                let next_xys = walk(m, x, y, r);
                all_next_xys.extend(next_xys.into_iter());
            }
            all_next_xys
        }
    }
}

fn gen_map(m: &[Vec<u8>]) -> (Vec<Vec<u8>>, usize, usize) {
    let mut minx = m[0].len();
    let mut maxx = 0;
    let mut miny = m.len();
    let mut maxy = 0;
    for (x, y) in iproduct!(0..m[0].len(), 0..m.len()) {
        if m[y][x] == b'#' {
            minx = minx.min(x);
            maxx = maxx.max(x);
            miny = miny.min(y);
            maxy = maxy.max(y);
        }
    }
    let nm = (miny..=maxy)
        .map(|y| {
            (minx..=maxx)
                .map(|x| {
                    let c = m[y][x];
                    if c == b'?' {
                        b'#'
                    } else {
                        c
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    (nm, 500 - minx, 500 - miny)
}

fn bfs(m: &[Vec<u8>], x: usize, y: usize) -> (i32, i32) {
    let mut saw = HashSet::new();
    saw.insert((x, y));
    let mut q = VecDeque::new();
    q.push_back((x, y, 0));
    let mut part1_cnt = 0;
    let mut part2_cnt = 0;
    while let Some((x, y, c)) = q.pop_front() {
        part1_cnt = part1_cnt.max(c);
        for &(nx, ny) in &[(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if saw.contains(&(nx, ny)) || m[ny][nx] == b'#' {
                continue;
            }
            let nc = c + (if m[ny][nx] == b'.' { 0 } else { 1 });
            if m[ny][nx] == b'.' && nc >= 1000 {
                part2_cnt += 1;
            }
            saw.insert((nx, ny));
            q.push_back((nx, ny, nc));
        }
    }
    (part1_cnt, part2_cnt)
}

fn main() {
    let input = common::get_input(20).unwrap();
    let input = input
        .bytes()
        .skip(1)
        .take_while(|&c| c != b'$')
        .collect_vec();
    let r = simplify(parse(&input, 0, input.len()));
    let mut m = vec![vec![b'?'; 1000]; 1000];
    let _ = walk(&mut m, 500, 500, &r);
    m[500][500] = b'X';
    let (m, x, y) = gen_map(&m);
    // println!("{}", m.iter().map(|v| String::from_utf8_lossy(v)).collect_vec().join("\n"));
    println!("{:?}", bfs(&m, x, y));
}
