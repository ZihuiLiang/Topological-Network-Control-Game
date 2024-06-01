use crate::functions::*;
use std::cmp::{max, min};
use std::collections::HashMap;

pub fn theorem4_solver(paths: Vec<i8>) -> bool {
    let cntp = get_path_cntp(paths.clone());
    let cntodd = get_path_cntodd(paths.clone());
    let s = get_path_s(paths.clone());
    if cntp[8] + cntp[9] == 0
        && cntodd & 1 == 0
        && cntp[1] + cntp[2] + cntp[3] + cntp[4] == 0
        && ((!s.contains(&0) && !s.contains(&5) && !s.contains(&6) && !s.contains(&7))
            || (s.contains(&0) && !s.contains(&5) && s.contains(&6) && !s.contains(&7)))
    {
        return false;
    }
    true
}

pub fn theorem4_compute(
    opt: &mut HashMap<((i8, i8), Vec<i8>), i8>,
    mut n: i8,
    mut m: i8,
    mut paths: Vec<i8>,
) -> i8 {
    paths.sort();
    if n > m {
        std::mem::swap(&mut n, &mut m);
    }
    if let Some(&ans) = opt.get(&((n, m), paths.clone())) {
        return ans;
    }
    let mut mx_opt = i8::MIN;
    if n == 0 && m == 0 {
        let mut _paths = vec![];
        for i in 0..paths.len() {
            let mut new_paths = _paths.clone();
            for j in i + 1..paths.len() {
                new_paths.push(paths[j]);
            }
            for j in 0..paths[i] {
                let mut v = 1;
                let mut _n = 0;
                let mut _m = 0;
                if j != 0 {
                    v += 1;
                    _n = (j - 1) as i8;
                }
                if j != paths[i] - 1 {
                    v += 1;
                    _m = (paths[i] - j - 2) as i8;
                }
                let _opt = theorem4_compute(opt, _n, _m, new_paths.clone());
                if mx_opt < v - _opt {
                    mx_opt = v - _opt;
                }
            }
            _paths.push(paths[i]);
        }
    } else {
        for i in 1..=min(2, n) {
            let mut v = i;
            if i < n {
                v += 1;
            }
            let mut _n = max(n - i - 1, 0);
            let _opt = theorem4_compute(opt, _n, m, paths.clone());
            if mx_opt < v - _opt {
                mx_opt = v - _opt;
            }
        }
        for i in 1..=min(2, m) {
            let mut v = i;
            if i < m {
                v += 1;
            }
            let mut _m = max(m - i - 1, 0);
            let _opt = theorem4_compute(opt, n, _m, paths.clone());
            if mx_opt < v - _opt {
                mx_opt = v - _opt;
            }
        }
    }
    opt.insert(((n, m), paths.clone()), mx_opt);
    mx_opt
}

pub fn theorem4_generate(
    opt: &mut HashMap<((i8, i8), Vec<i8>), i8>,
    paths: Vec<i8>,
    size: i8,
    max_size: i8,
) {
    theorem4_compute(opt, 0, 0, paths.clone());
    for i in max_size..=size {
        let mut new_paths = paths.clone();
        new_paths.push(i);
        theorem4_generate(opt, new_paths, size - i, i);
    }
}

pub fn theorem4_verifier() {
    println!("Verifying Theorem 4 for |G|<=60 ..");
    let mut opt = HashMap::new();
    opt.insert(((0, 0), vec![]), 0);
    theorem4_generate(&mut opt, vec![], 60, 1);
    let mut passed = true;
    for config in opt.keys() {
        if config.0 .0 == 0
            && config.0 .1 == 0
            && theorem4_solver(config.1.clone()) != (opt[config] > 0)
        {
            println!(
                "Counter case: paths={:?} solver answer is {} true answer is {}",
                config.1,
                theorem4_solver(config.1.clone()),
                opt[config] > 0
            );
            passed = false;
        }
    }
    if passed {
        println!("Theorem 4 for |G|<=60 passed!");
    } else {
        println!("Theorem 4 for |G|<=60 failed!");
    }
}
