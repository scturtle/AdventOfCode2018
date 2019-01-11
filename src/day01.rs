mod common;

use itertools::Itertools;

fn main() {
    let changes = common::get_input(1)
        .trim_right()
        .split('\n')
        .map(|s| s.parse::<i32>().unwrap())
        .collect_vec();
    println!("{}", changes.iter().sum::<i32>());
    let mut saw = std::collections::HashSet::new();
    saw.insert(0);
    let mut freq = 0;
    loop {
        for change in &changes {
            freq += change;
            if !saw.insert(freq) {
                println!("{}", freq);
                return;
            }
        }
    }
}
