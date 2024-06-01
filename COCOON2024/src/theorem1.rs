use crate::functions::*;
use std::cmp::{max, min};
use std::collections::HashMap;
pub fn theorem1_solver(cycles: Vec<i8>) -> i8 {
    let cntp = get_cycle_cntp(cycles.clone());
    assert!(cntp[10] == 0 && cntp[11] == 0 && cntp[12] == 0);
    let cntodd = get_cycle_cntodd(cycles.clone());
    let s = get_cycle_s(cycles.clone());
    if cntodd & 1 == 1 {
        if cntp[3] > 0 {
            return 3;
        }
        if (!s.contains(&1) && !s.contains(&5) && !s.contains(&7) && s.contains(&9))
            || (s.contains(&1) && !s.contains(&5) && s.contains(&7) && s.contains(&9))
        {
            if cntp[5] > 0 {
                if s.contains(&4) {
                    return 3;
                } else {
                    return 1;
                }
            }
            if (!s.contains(&4) && !s.contains(&8)) || (s.contains(&4) && s.contains(&8)) {
                return 3;
            } else {
                return 1;
            }
        } else {
            if s.contains(&4) {
                return 3;
            } else {
                return 1;
            }
        }
    } else {
        if cntp[2] + cntp[3] + cntp[4] + cntp[5] == 0
            && ((!s.contains(&1) && !s.contains(&7) && !s.contains(&8) && !s.contains(&9))
                || (s.contains(&1) && s.contains(&7) && !s.contains(&8) && !s.contains(&9)))
        {
            return 0;
        } else {
            return 2;
        }
    }
}

pub fn theorem1_compute(opt: &mut HashMap<(i8, Vec<i8>), i8>, n: i8, mut cycles: Vec<i8>) -> i8 {
    cycles.sort();
    if let Some(&ans) = opt.get(&(n, cycles.clone())) {
        return ans;
    }
    let mut mx_opt = i8::MIN;
    if n == 0 {
        let mut _cycles = vec![];
        for i in 0..cycles.len() {
            let mut new_cycles = _cycles.clone();
            for j in i + 1..cycles.len() {
                new_cycles.push(cycles[j]);
            }
            let v = min(3, cycles[i]);
            let _n = max(cycles[i] - 3, 0);
            let _opt = theorem1_compute(opt, _n, new_cycles.clone());
            if mx_opt < v - _opt {
                mx_opt = v - _opt;
            }
            _cycles.push(cycles[i]);
        }
    } else {
        for i in 1..=min(2, n) {
            let mut v = i;
            if i < n {
                v += 1;
            }
            let mut _n = max(n - i - 1, 0);
            let _opt = theorem1_compute(opt, _n, cycles.clone());
            if mx_opt < v - _opt {
                mx_opt = v - _opt;
            }
        }
    }
    opt.insert((n, cycles.clone()), mx_opt);
    mx_opt
}

pub fn theorem1_generate(
    opt: &mut HashMap<(i8, Vec<i8>), i8>,
    cycles: Vec<i8>,
    size: i8,
    max_size: i8,
) {
    theorem1_compute(opt, 0, cycles.clone());
    for i in max_size..=size {
        let mut new_cycles = cycles.clone();
        if cycle_p_type(i) <= 9 {
            new_cycles.push(i);
            theorem1_generate(opt, new_cycles, size - i, i);
        }
    }
}

pub fn theorem1_verifier() {
    println!("Verifying Theorem 1 for 1<=|G|<=80 ..");
    let mut opt = HashMap::new();
    opt.insert((0, vec![]), 0);
    theorem1_generate(&mut opt, vec![], 80, 1);
    let mut passed = true;
    for config in opt.keys() {
        if config.0 == 0 && theorem1_solver(config.1.clone()) != opt[config] {
            println!(
                "Counter case: cycles={:?} solver answer is {} true answer is {}",
                config.1,
                theorem1_solver(config.1.clone()),
                opt[config]
            );
            passed = false;
        }
    }
    if passed {
        println!("Theorem 1 for |G|<=80 passed!");
    } else {
        println!("Theorem 1 for |G|<=80 failed!");
    }
}
