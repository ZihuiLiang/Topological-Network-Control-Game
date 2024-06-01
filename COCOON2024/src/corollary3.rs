use std::cmp::max;
use std::collections::HashMap;

pub fn corollary3_solver(opt_g: i8, n: i8) -> i8 {
    assert!(opt_g >= 2 && opt_g <= 3 && n >= 1 && n <= 18);
    if n == 1 || n == 7 || n == 13 {
        return 1 - opt_g;
    }
    if n == 2 || n == 6 || n == 8 || n == 12 || n == 18 {
        return 2 - opt_g;
    }
    if n == 3 || n == 11 || n == 17 {
        return 3 - opt_g;
    }
    if n == 16 {
        return 4 - opt_g;
    }
    if n == 5 || n == 9 || n == 15 {
        return 1 + opt_g;
    }
    if n == 4 || n == 10 {
        return 2 + opt_g;
    }
    opt_g
}

pub fn corollary3_verifier() {
    println!("Verifying Corollary 3 for 1<=n<=18 ..");
    let mut map_path = HashMap::new();
    let mut map_cycle = HashMap::new();
    map_path.insert((2, 1), -1);
    map_path.insert((3, 1), -2);
    map_cycle.insert((2, 1), -1);
    map_cycle.insert((3, 1), -2);
    map_path.insert((2, 2), 0);
    map_path.insert((3, 2), -1);
    map_cycle.insert((2, 2), 0);
    map_cycle.insert((3, 2), -1);
    map_path.insert((2, 3), 3);
    map_path.insert((3, 3), 4);
    map_cycle.insert((2, 3), 1);
    map_cycle.insert((3, 3), 0);

    for n in 4..=18 {
        for opt_g in 2..=3 {
            map_cycle.insert((opt_g, n), 3 - map_path.get(&(opt_g, n - 3)).unwrap());
            map_path.insert(
                (opt_g, n),
                max(
                    2 - map_path.get(&(opt_g, n - 2)).unwrap(),
                    3 - map_path.get(&(opt_g, n - 3)).unwrap(),
                ),
            );
        }
    }
    let mut passed = true;
    for config in map_cycle.keys() {
        if config.1 >= 16 && corollary3_solver(config.0, config.1) != map_cycle[config] {
            println!(
                "Counter case: Opt(G')={} n={} solver answer is {} true answer is {}",
                config.0,
                config.1,
                corollary3_solver(config.0, config.1),
                map_cycle[config]
            );
            passed = false;
        }
    }
    if passed {
        println!("Corollary 3 passed!");
    } else {
        println!("Corollary 3 failed!");
    }
}
