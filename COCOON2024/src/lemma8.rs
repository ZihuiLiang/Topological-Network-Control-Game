use std::cmp::{max, min};
use std::collections::HashMap;

pub fn lemma8_solver(opt_g: i8, n: i8) -> i8 {
    assert!(opt_g >= 2 && opt_g <= 3);
    if n > 15 {
        match n % 6 {
            0 => return -2 + opt_g,
            1 => return -1 + opt_g,
            2 => return opt_g,
            3 => return 5 - opt_g,
            4 => return 4 - opt_g,
            5 => return 3 - opt_g,
            _ => panic!(),
        
        }
    }
    if n == 7 || n == 11 {
        return 3 - opt_g;
    }
    if n == 2 || n == 6 || n == 12 {
        return 2 - opt_g;
    }
    if n == 1 {
        return 1 - opt_g;
    }
    if n == 13 {
        return -1 + opt_g;
    }
    if n == 8 || n == 14 {
        return opt_g;
    }
    if n == 3 || n == 5 || n == 9 || n == 15 {
        return 1 + opt_g;
    }
    if n == 4 || n == 10 {
        return 2 + opt_g;
    }
    panic!();
}

pub fn lemma8_verifier() {
    println!("Verifying Lemma 8 for 1<=n<=33 ..");
    let mut map_path = HashMap::new();
    let mut map_two_path = HashMap::new();
    map_path.insert((2, 0), 2);
    map_path.insert((3, 0), 3);
    map_two_path.insert((2, 0, 0), 2);
    map_two_path.insert((3, 0, 0), 3);

    for n in 0..=33 {
        for opt_g in 2..=3 {
            for m in 0..=33 {
                if m > 0 || n > 0 {
                    let mut mx_opt = i8::MIN;
                    for i in 2..=3 {
                        let v_n = if n == 0 {
                            i8::MIN
                        } else {
                            min(i, n) - map_two_path.get(&(opt_g, max(0, n - i), m)).unwrap()
                        };
                        let v_m = if m == 0 {
                            i8::MIN
                        } else {
                            min(i, m) - map_two_path.get(&(opt_g, n, max(0, m - i))).unwrap()
                        };
                        mx_opt = max(mx_opt, max(v_n, v_m));
                    }
                    map_two_path.insert((opt_g, n, m), mx_opt);
                }
            }
            if n > 0 {
                let mut mx_opt = i8::MIN;
                for i in 0..n {
                    let _n = max(0, i - 1);
                    let _m = max(0, n - i - 2);
                    let mut v = 1;
                    if i > 0 {
                        v += 1;
                    }
                    if i < n - 1 {
                        v += 1;
                    }
                    mx_opt = max(mx_opt, v - map_two_path.get(&(opt_g, _n, _m)).unwrap());
                }
                map_path.insert((opt_g, n), mx_opt);
            }
        }
    }
    let mut passed = true;
    for config in map_path.keys() {
        if config.1 > 0 && lemma8_solver(config.0, config.1) != map_path[config] {
            println!(
                "Counter case: Opt(G')={} n={} solver answer is {} true answer is {}",
                config.0,
                config.1,
                lemma8_solver(config.0, config.1),
                map_path[config]
            );
            passed = false;
        }
    }
    if passed {
        println!("Lemma 8 passed!");
    } else {
        println!("Lemma 8 failed!");
    }
}
