use itertools::Itertools;

fn main() {
    let input = common::get_input(2).unwrap();
    let ids = input.trim_end().split('\n').collect_vec();
    let mut cnt2 = 0;
    let mut cnt3 = 0;
    for id in &ids {
        let mut cs = id.chars().collect_vec();
        cs.sort();
        let mut has2 = false;
        let mut has3 = false;
        for (_, g) in &cs.iter().group_by(|&c| c) {
            match g.count() {
                2 => has2 = true,
                3 => has3 = true,
                _ => {}
            }
        }
        cnt2 += if has2 { 1 } else { 0 };
        cnt3 += if has3 { 1 } else { 0 };
    }
    println!("{}", cnt2 * cnt3);

    for i in 0..ids.len() - 1 {
        for j in i + 1..ids.len() {
            let (same, diff): (Vec<_>, _) = ids[i]
                .chars()
                .zip(ids[j].chars())
                .partition(|(a, b)| a == b);
            if diff.len() == 1 {
                println!("{}", same.iter().map(|&(c, _)| c).collect::<String>());
                return;
            }
        }
    }
}
