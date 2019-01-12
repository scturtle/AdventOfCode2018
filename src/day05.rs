use itertools::Itertools;

fn react(a: char, b: char) -> bool {
    a != b && a.to_ascii_lowercase() == b.to_ascii_lowercase()
}

fn fully_react(polymer: &mut Vec<char>) {
    let mut last_len = -1;
    while last_len != polymer.len() as i32 {
        last_len = polymer.len() as i32;
        let mut i: i32 = -1;
        for j in 0..polymer.len() {
            if i == -1 {
                i += 1;
                polymer[i as usize] = polymer[j];
            } else if react(polymer[i as usize], polymer[j]) {
                i -= 1;
            } else {
                i += 1;
                polymer[i as usize] = polymer[j];
            }
        }
        polymer.resize((i + 1) as usize, ' ');
    }
}

fn main() {
    let input = common::get_input(5).unwrap();
    let polymer = input.trim_end().chars().collect_vec();
    let mut orig = polymer.clone();
    fully_react(&mut orig);
    println!("{:?}", orig.len());
    let res = (b'a'..=b'z')
        .map(|u| {
            let mut p = polymer
                .iter()
                .cloned()
                .filter(|&c| c.to_ascii_lowercase() != u as char)
                .collect_vec();
            fully_react(&mut p);
            p.len()
        })
        .min()
        .unwrap();
    println!("{}", res);
}
