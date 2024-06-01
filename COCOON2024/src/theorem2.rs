use crate::functions::*;
use std::cmp::{max, min};
use std::collections::HashMap;

pub fn theorem2_solver(cycles: Vec<i8>) -> bool {
    let cntp = get_cycle_cntp(cycles.clone());
    let cntodd = get_cycle_cntodd(cycles.clone());
    let s = get_cycle_s(cycles.clone());
    if cntodd & 1 == 0
        && ((!s.contains(&1) && !s.contains(&7) && !s.contains(&8) && !s.contains(&9))
            || (s.contains(&1) && s.contains(&7) && !s.contains(&8) && !s.contains(&9)))
        && cntp[2] + cntp[3] + cntp[4] + cntp[5] + cntp[10] + cntp[11] + cntp[12] == 0
    {
        return false;
    }
    true
}

pub fn theorem2_compute(opt: &mut HashMap<(i8, Vec<i8>), i8>, n: i8, mut cycles: Vec<i8>) -> i8 {
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
            let _opt = theorem2_compute(opt, _n, new_cycles.clone());
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
            let _opt = theorem2_compute(opt, _n, cycles.clone());
            if mx_opt < v - _opt {
                mx_opt = v - _opt;
            }
        }
    }
    opt.insert((n, cycles.clone()), mx_opt);
    mx_opt
}

pub fn theorem2_generate(
    opt: &mut HashMap<(i8, Vec<i8>), i8>,
    cycles: Vec<i8>,
    size: i8,
    max_size: i8,
) {
    theorem2_compute(opt, 0, cycles.clone());
    for i in max_size..=size {
        let mut new_cycles = cycles.clone();
        new_cycles.push(i);
        theorem2_generate(opt, new_cycles, size - i, i);
    }
}

pub fn theorem2_verifier() {
    println!("Verifying Theorem 2 for 1<=|G|<=60 ..");
    let mut opt = HashMap::new();
    opt.insert((0, vec![]), 0);
    theorem2_generate(&mut opt, vec![], 60, 1);
    let mut passed = true;
    theorem2_solver(vec![1, 1, 1, 1, 4]);
    for config in opt.keys() {
        if config.0 == 0 && theorem2_solver(config.1.clone()) != (opt[config] > 0) {
            println!(
                "Counter case: cycles={:?} solver answer is {} true answer is {}",
                config.1,
                theorem2_solver(config.1.clone()),
                opt[config] > 0
            );
            passed = false;
        }
    }
    if passed {
        println!("Theorem 2 for |G|<=60 passed!");
    } else {
        println!("Theorem 2 for |G|<=60 failed!");
    }
}
