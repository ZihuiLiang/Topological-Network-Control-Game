use crate::functions::*;
use std::cmp::{max, min};
use std::collections::HashMap;

pub fn theorem3_solver(paths: Vec<i8>) -> i8 {
    let cntp = get_path_cntp(paths.clone());
    assert!(cntp[8] == 0 && cntp[9] == 0);
    let cntodd = get_path_cntodd(paths.clone());
    let s = get_path_s(paths.clone());
    if cntodd & 1 == 1 {
        if cntp[3] > 0 {
            return 3;
        }
        if cntp[1] > 1 {
            if s.contains(&4) {
                return 3;
            } else {
                return 1;
            }
        } else {
            if s.contains(&7) {
                if s.contains(&4) {
                    return 3;
                } else {
                    return 1;
                }
            }
            let mut cnt046 = 0;
            if s.contains(&0) {
                cnt046 += 1;
            }
            if s.contains(&4) {
                cnt046 += 1;
            }
            if s.contains(&6) {
                cnt046 += 1;
            }
            if cnt046 == 0 || cnt046 == 2 {
                return 3;
            } else {
                return 1;
            }
        }
    } else {
        if cntp[1] + cntp[2] + cntp[3] + cntp[4] == 0
            && ((!s.contains(&0) && !s.contains(&5) && !s.contains(&6) && !s.contains(&7))
                || (s.contains(&0) && !s.contains(&5) && s.contains(&6) && !s.contains(&7)))
        {
            return 0;
        } else {
            return 2;
        }
    }
}

pub fn theorem3_compute(
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
                let _opt = theorem3_compute(opt, _n, _m, new_paths.clone());
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
            let _opt = theorem3_compute(opt, _n, m, paths.clone());
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
            let _opt = theorem3_compute(opt, n, _m, paths.clone());
            if mx_opt < v - _opt {
                mx_opt = v - _opt;
            }
        }
    }
    opt.insert(((n, m), paths.clone()), mx_opt);
    mx_opt
}

pub fn theorem3_generate(
    opt: &mut HashMap<((i8, i8), Vec<i8>), i8>,
    paths: Vec<i8>,
    size: i8,
    max_size: i8,
) {
    theorem3_compute(opt, 0, 0, paths.clone());
    for i in max_size..=size {
        let mut new_paths = paths.clone();
        if path_p_type(i) <= 7 {
            new_paths.push(i);
            theorem3_generate(opt, new_paths, size - i, i);
        }
    }
}

pub fn theorem3_verifier() {
    println!("Verifying Theorem 3 for |G|<=75 ..");
    let mut opt = HashMap::new();
    opt.insert(((0, 0), vec![]), 0);
    theorem3_generate(&mut opt, vec![], 75, 1);
    let mut passed = true;
    for config in opt.keys() {
        if config.0 .0 == 0 && config.0 .1 == 0 && theorem3_solver(config.1.clone()) != opt[config]
        {
            println!(
                "Counter case: paths={:?} solver answer is {} true answer is {}",
                config.1,
                theorem3_solver(config.1.clone()),
                opt[config]
            );
            passed = false;
        }
    }
    if passed {
        println!("Theorem 3 for |G|<=75 passed!");
    } else {
        println!("Theorem 3 for |G|<=75 failed!");
    }
}
