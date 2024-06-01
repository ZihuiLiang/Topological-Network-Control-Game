use std::cmp::{max, min};
use std::collections::HashMap;

pub fn lemma7_solver(opt_g: i8, n: i8, m: i8) -> i8 {
    assert!(opt_g >= 2 && opt_g <= 3);
    if n >= 6 || m >= 6 {
        return lemma7_solver(opt_g, n % 6, m % 6);
    }
    if n > m {
        return lemma7_solver(opt_g, m, n);
    }
    match (m, n) {
        (0, 0) => -2 + opt_g,
        (1, 0) => -1 + opt_g,
        (2, 0) => opt_g,
        (3, 0) => 5 - opt_g,
        (4, 0) => 4 - opt_g,
        (5, 0) => 3 - opt_g,
        (1, 1) => opt_g,
        (2, 1) => 5 - opt_g,
        (3, 1) => 4 - opt_g,
        (4, 1) => 3 - opt_g,
        (5, 1) => -2 + opt_g,
        (2, 2) => 4 - opt_g,
        (3, 2) => 3 - opt_g,
        (4, 2) => -2 + opt_g,
        (5, 2) => -1 + opt_g,
        (3, 3) => -2 + opt_g,
        (4, 3) => -1 + opt_g,
        (5, 3) => opt_g,
        (4, 4) => opt_g,
        (5, 4) => 5 - opt_g,
        (5, 5) => 4 - opt_g,
        _ => {
            panic!()
        }
    }
}

pub fn lemma7_verifier() {
    println!("Verifying Lemma 7 for 16<=n<=25 and m<=25 ..");
    let mut map = HashMap::new();
    map.insert((2, 0, 0), 2);
    map.insert((3, 0, 0), 3);
    for n in 0..=25 {
        for m in 0..=25 {
            if n == 0 && m == 0 {
                continue;
            }
            for opt_g in 2..=3 {
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
        if config.1 >= 16 && lemma7_solver(config.0, config.1, config.2) != map[config] {
            println!(
                "Counter case: Opt(G')={} n={} m={} solver answer is {} true answer is {}",
                config.0,
                config.1,
                config.2,
                lemma7_solver(config.0, config.1, config.2),
                map[config]
            );
            passed = false;
        }
    }
    if passed {
        println!("Lemma 7 passed!");
    } else {
        println!("Lemma 7 failed!");
    }
}
