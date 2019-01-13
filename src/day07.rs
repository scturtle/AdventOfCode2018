use itertools::Itertools;

fn main() {
    let orders = common::get_input(7)
        .unwrap()
        .trim_end()
        .lines()
        .map(|s| {
            let cs = s.split(' ').collect_vec();
            (cs[1].as_bytes()[0], cs[7].as_bytes()[0])
        })
        .collect_vec();
    let all_works = {
        let mut all_works = Vec::new();
        for t in &orders {
            all_works.push(t.0);
            all_works.push(t.1);
        }
        all_works.sort();
        all_works.dedup();
        all_works
    };
    let mut done = Vec::new();
    while all_works.len() != done.len() {
        for w in &all_works {
            if done.contains(w) {
                continue;
            }
            let ok = orders.iter().all(|t| &t.1 != w || done.contains(&t.0));
            if ok {
                done.push(*w);
                break;
            }
        }
    }
    println!("{}", std::str::from_utf8(&done).unwrap());

    let mut ts: u32 = 0;
    let mut workers: Vec<(u8, u8)> = vec![(b' ', 0); 5]; // (work, time_left)
    done.clear();
    while all_works.len() != done.len() {
        let passed = workers
            .iter()
            // do not count idle worker
            .filter_map(|t| if t.1 != 0 { Some(t.1) } else { None })
            .min()
            .unwrap_or(0);
        ts += u32::from(passed);

        for e in &mut workers {
            // skip idle worker
            if e.1 == 0 {
                continue;
            }
            e.1 -= passed;
            if e.1 == 0 && e.0 != 0 {
                done.push(e.0);
            }
        }

        for w in &all_works {
            // done or working
            if done.contains(w) || workers.iter().any(|e| &e.0 == w) {
                continue;
            }
            let ok = orders.iter().all(|t| &t.1 != w || done.contains(&t.0));
            if ok {
                for e in &mut workers {
                    // assign to worker
                    if e.1 == 0 {
                        e.0 = *w;
                        e.1 = w - 4;
                        break;
                    }
                }
            }
        }
    }
    println!("{}", ts);
}
