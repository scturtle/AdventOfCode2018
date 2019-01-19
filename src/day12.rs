use itertools::Itertools;

fn strtoidx(s: &str) -> usize {
    let arr = s.bytes().take(5).map(|c| c == b'#').collect_vec();
    toidx(&arr)
}

fn toidx(v: &[bool]) -> usize {
    (0..5)
        .map(|i| if v[i] { 1 << (4 - i) } else { 0 })
        .sum::<usize>()
}

fn next(tran: &[bool], v: &[bool]) -> Vec<bool> {
    let mut next = vec![false; v.len()];
    for i in 2..v.len() - 2 {
        next[i] = tran[toidx(&v[i - 2..i + 3])];
    }
    next
}

#[allow(dead_code)]
fn show(v: &[bool]) -> String {
    v.iter().map(|&b| if b { '#' } else { '.' }).collect()
}

fn count(padding: usize, v: &[bool]) -> i32 {
    v.iter()
        .enumerate()
        .map(|(i, &b)| if b { i as i32 - padding as i32 } else { 0 })
        .sum::<i32>()
}

fn main() {
    let input = common::get_input(12).unwrap();
    let mut lines = input.lines();
    let init = lines.next().unwrap().split(' ').nth(2).unwrap();
    let _ = lines.next();
    let tran = {
        let mut tran = [false; 32];
        for l in lines {
            tran[strtoidx(l)] = l.as_bytes()[9] == b'#';
        }
        tran
    };
    let padding = 300;
    let origin = {
        let mut state = vec![false; padding];
        state.append(&mut init.bytes().map(|c| c == b'#').collect_vec());
        state.append(&mut vec![false; padding]);
        state
    };
    let mut state = origin.clone();
    for _ in 0..20 {
        state = next(&tran, &state);
    }
    println!("{}", count(padding, &state));

    state = origin.clone();
    let mut remain = 50_000_000_000u64;
    while remain > 0 {
        state = next(&tran, &state);
        remain -= 1;
        let c = count(padding, &state);
        if c > 12000 {
            let next_c = count(padding, &next(&tran, &state));
            println!("{}", c as u64 + remain * (next_c - c) as u64);
            return;
        }
    }
}
