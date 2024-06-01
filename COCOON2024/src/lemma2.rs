use std::cmp::{max, min};
use std::collections::HashMap;

pub fn lemma2_solver(opt_g: i8, n: i8) -> i8 {
    assert!(opt_g >= 2 && opt_g <= 3 && n >= 16);
    match n % 6 {
        0 => -2 + opt_g,
        1 => -1 + opt_g,
        2 => opt_g,
        3 => 5 - opt_g,
        4 => 4 - opt_g,
        5 => 3 - opt_g,
        _ => 0,
    }
}

pub fn lemma2_verifier() {
    println!("Verifying Lemma 2 for 16<=n<=100 ..");
    let mut map = HashMap::new();
    map.insert((2, 0), 2);
    map.insert((3, 0), 3);
    for n in 1..=100 {
        for opt_g in 2..=3 {
            let mut mx_opt = i8::MIN;
            for i in 1..=min(2, n) {
                let mut v = i;
                if i < n {
                    v += 1;
                }
                let mut _n = max(n - i - 1, 0);
                let _opt = map[&(opt_g, _n)];
                if i == 1 || mx_opt < v - _opt {
                    mx_opt = v - _opt;
                }
            }
            map.insert((opt_g, n), mx_opt);
        }
    }
    let mut passed = true;
    for config in map.keys() {
        if config.1 >= 16 && lemma2_solver(config.0, config.1) != map[config] {
            println!(
                "Counter case: Opt(G')={} n={} solver answer is {} true answer is {}",
                config.0,
                config.1,
                lemma2_solver(config.0, config.1),
                map[config]
            );
            passed = false;
        }
    }
    if passed {
        println!("Lemma 2 passed!");
    } else {
        println!("Lemma 2 failed!");
    }
}
