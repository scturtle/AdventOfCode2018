use itertools::Itertools;
use std::collections::HashMap;

fn addr(regs: &mut [usize], a: usize, b: usize, c: usize) {
    regs[c] = regs[a] + regs[b];
}

fn addi(regs: &mut [usize], a: usize, b: usize, c: usize) {
    regs[c] = regs[a] + b;
}

fn mulr(regs: &mut [usize], a: usize, b: usize, c: usize) {
    regs[c] = regs[a] * regs[b];
}

fn muli(regs: &mut [usize], a: usize, b: usize, c: usize) {
    regs[c] = regs[a] * b;
}

fn banr(regs: &mut [usize], a: usize, b: usize, c: usize) {
    regs[c] = regs[a] & regs[b];
}

fn bani(regs: &mut [usize], a: usize, b: usize, c: usize) {
    regs[c] = regs[a] & b;
}

fn borr(regs: &mut [usize], a: usize, b: usize, c: usize) {
    regs[c] = regs[a] | regs[b];
}

fn bori(regs: &mut [usize], a: usize, b: usize, c: usize) {
    regs[c] = regs[a] | b;
}

fn setr(regs: &mut [usize], a: usize, _: usize, c: usize) {
    regs[c] = regs[a];
}

fn seti(regs: &mut [usize], a: usize, _: usize, c: usize) {
    regs[c] = a;
}

fn gtir(regs: &mut [usize], a: usize, b: usize, c: usize) {
    regs[c] = (a > regs[b]) as usize;
}

fn gtri(regs: &mut [usize], a: usize, b: usize, c: usize) {
    regs[c] = (regs[a] > b) as usize;
}

fn gtrr(regs: &mut [usize], a: usize, b: usize, c: usize) {
    regs[c] = (regs[a] > regs[b]) as usize;
}

fn eqir(regs: &mut [usize], a: usize, b: usize, c: usize) {
    regs[c] = (a == regs[b]) as usize;
}

fn eqri(regs: &mut [usize], a: usize, b: usize, c: usize) {
    regs[c] = (regs[a] == b) as usize;
}

fn eqrr(regs: &mut [usize], a: usize, b: usize, c: usize) {
    regs[c] = (regs[a] == regs[b]) as usize;
}

fn main() {
    let input = common::get_input(16).unwrap();
    let parts = input.split("\n\n\n").collect_vec();
    assert!(parts.len() == 2);
    let part1 = parts[0].lines().collect_vec();
    let mut idx = 0;
    let pat = regex::Regex::new(r"\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    let mut compat = vec![vec![true; 16]; 16]; // code -> op
    let op_fns = [
        addr, addi, mulr, muli, banr, bani, borr, bori, //
        setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr,
    ];
    let mut part1_result = 0;
    while idx + 2 < part1.len() {
        let im = pat.captures(part1[idx]).unwrap();
        let is = (1..=4)
            .map(|i| im[i].parse::<usize>().unwrap())
            .collect_vec();
        let ops = part1[idx + 1]
            .split_whitespace()
            .map(|c| c.parse().unwrap())
            .collect_vec();
        let om = pat.captures(part1[idx + 2]).unwrap();
        let os = (1..=4)
            .map(|i| om[i].parse::<usize>().unwrap())
            .collect_vec();
        idx += 4;
        // check
        let mut cur = 0;
        for (op_i, op_fn) in op_fns.iter().enumerate() {
            let mut is = is.clone();
            op_fn(&mut is, ops[1], ops[2], ops[3]);
            if is == os {
                cur += 1;
            } else {
                compat[ops[0]][op_i] = false;
            }
        }
        if cur >= 3 {
            part1_result += 1;
        }
    }
    dbg!(part1_result);
    let rel = loop {
        let cols = (0..16)
            .filter_map(|i| {
                let js = (0..16).filter(|&j| compat[j][i]).collect_vec();
                if js.len() == 1 {
                    Some((js[0], i))
                } else {
                    None
                }
            })
            .collect_vec();
        if cols.len() == 16 {
            break cols; // all solved
        }
        for (j, i) in cols {
            compat[j] = (0..16).map(|ii| ii == i).collect();
        }
    };
    // code => op index
    let rel: HashMap<_, _> = rel.into_iter().collect();
    let mut regs = vec![0; 4];
    for l in parts[1].lines().skip(1) {
        let ops = l
            .split_whitespace()
            .map(|c| c.parse().unwrap())
            .collect_vec();
        let op_i = rel[&ops[0]];
        op_fns[op_i](&mut regs, ops[1], ops[2], ops[3]);
    }
    println!("{}", regs[0]);
}
