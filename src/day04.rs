use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = common::get_input(4).unwrap();
    let mut records = input.trim_end().split('\n').collect_vec();
    records.sort();
    let guards: HashMap<i32, Vec<(i32, i32)>> = {
        let mut guards = HashMap::new();
        let mut current_id = 0;
        let mut current_start = 0;
        for r in records {
            let r = r.split(' ').collect_vec();
            let current_time: i32 = r[1][3..5].parse().unwrap();
            match r[2] {
                "Guard" => {
                    current_id = r[3][1..].parse().unwrap();
                }
                "falls" => {
                    current_start = current_time;
                }
                "wakes" => {
                    guards
                        .entry(current_id)
                        .or_insert_with(|| vec![])
                        .push((current_start, current_time));
                }
                _ => unreachable!(),
            }
        }
        guards
    };
    let (id, ds) = guards
        .iter()
        .max_by_key(|(_, ds)| ds.iter().map(|(s, e)| e - s).sum::<i32>())
        .unwrap();

    let most_freq_minute = |ds: &Vec<(i32, i32)>| -> (i32, i32) {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        for d in ds {
            for m in d.0..d.1 {
                *cnt.entry(m).or_insert(0) += 1;
            }
        }
        cnt.into_iter().max_by_key(|&(_, t)| t).unwrap()
    };

    let ts = most_freq_minute(ds).0;
    println!("{}", id * ts);

    let res = guards
        .iter()
        .map(|(id, ds)| (id, most_freq_minute(ds)))
        .max_by_key(|&(_, (_, cnt))| cnt)
        .unwrap();
    println!("{}", res.0 * (res.1).0);
}
