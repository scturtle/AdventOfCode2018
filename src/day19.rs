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
    let input = common::get_input(19).unwrap();
    let mut input = input.lines();
    let reg_ip: usize = input
        .next()
        .unwrap()
        .split(' ')
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let op_names = [
        "addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", //
        "setr", "seti", "gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr",
    ];
    let op_fns = [
        addr, addi, mulr, muli, banr, bani, borr, bori, //
        setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr,
    ];
    let ops: HashMap<_, _> = op_names.iter().zip(op_fns.iter()).collect();
    let mut regs = [0usize; 6];
    let part2 = true;
    if part2 {
        regs[0] = 1;
    }
    let insts = input
        .map(|l| {
            let l = l.split(' ').collect_vec();
            let op = l[0];
            let a = l[1].parse().unwrap();
            let b = l[2].parse().unwrap();
            let c = l[3].parse().unwrap();
            (op, a, b, c)
        })
        .collect_vec();
    let mut ip = 0;
    loop {
        if ip >= insts.len() {
            break;
        }
        if part2 && ip == 1 {
            // part 2
            println!(
                "part2: {}",
                (1..=regs[2]).filter(|i| regs[2] % i == 0).sum::<usize>()
            );
            return;
        }
        let (op, a, b, c) = insts[ip];
        regs[reg_ip] = ip;
        ops[&op](&mut regs, a, b, c);
        // println!("{}: {:?}", ip, regs);
        ip = regs[reg_ip] + 1;
    }
    println!("part1: {}", regs[0]);
}
