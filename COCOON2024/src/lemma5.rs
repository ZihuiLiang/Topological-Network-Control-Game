use std::cmp::{max, min};
use std::collections::HashMap;

pub fn lemma5_solver(opt_g: i8, n: i8, m: i8) -> i8 {
    if n >= 6 || m >= 6 {
        return lemma5_solver(opt_g, n % 6, m % 6);
    }
    if n > m {
        return lemma5_solver(opt_g, m, n);
    }
    match (m, n) {
        (0, 0) => opt_g,
        (1, 0) => 1 - opt_g,
        (2, 0) => 2 - opt_g,
        (3, 0) => 3 - opt_g,
        (4, 0) => 2 + opt_g,
        (5, 0) => 1 + opt_g,
        (1, 1) => opt_g,
        (2, 1) => 1 + opt_g,
        (3, 1) => 2 + opt_g,
        (4, 1) => 3 - opt_g,
        (5, 1) => 2 - opt_g,
        (2, 2) => opt_g,
        (3, 2) => 1 + opt_g,
        (4, 2) => 2 - opt_g,
        (5, 2) => 3 - opt_g,
        (3, 3) => opt_g,
        (4, 3) => 1 - opt_g,
        (5, 3) => 2 - opt_g,
        (4, 4) => opt_g,
        (5, 4) => 1 + opt_g,
        (5, 5) => opt_g,
        _ => {
            panic!()
        }
    }
}

pub fn lemma5_verifier() {
    println!("Verifying Lemma 5 for 1<=n,m<=10 ..");
    let mut map = HashMap::new();
    map.insert((0, 0, 0), 0);
    map.insert((1, 0, 0), 1);
    for n in 0..=10 {
        for m in 0..=10 {
            if n == 0 && m == 0 {
                continue;
            }
            for opt_g in 0..=1 {
                let mut mx_opt = i8::MIN;
                for i in 2..=3 {
                    let v_n = if n == 0 {
                        i8::MIN
                    } else {
                        min(i, n) - map.get(&(opt_g, max(0, n - i), m)).unwrap()
                    };
                    let v_m = if m == 0 {
                        i8::MIN
                    } else {
                        min(i, m) - map.get(&(opt_g, n, max(0, m - i))).unwrap()
                    };
                    mx_opt = max(mx_opt, max(v_n, v_m));
                }
                map.insert((opt_g, n, m), mx_opt);
            }
        }
    }
    let mut passed = true;
    for config in map.keys() {
        if lemma5_solver(config.0, config.1, config.2) != map[config] {
            println!(
                "Counter case: Opt(G')={} n={} m={} solver answer is {} true answer is {}",
                config.0,
                config.1,
                config.2,
                lemma5_solver(config.0, config.1, config.2),
                map[config]
            );
            passed = false;
        }
    }
    if passed {
        println!("Lemma 5 passed!");
    } else {
        println!("Lemma 5 failed!");
    }
}
