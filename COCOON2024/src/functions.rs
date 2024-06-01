use std::collections::HashSet;

pub fn path_p_type(path: i8) -> usize {
    if path >= 16 {
        return (path % 6) as usize;
    } else {
        if path == 13 {
            return 1;
        }
        if path == 8 || path == 14 {
            return 2;
        }
        if path == 7 || path == 11 {
            return 5;
        }
        if path == 2 || path == 6 || path == 12 {
            return 6;
        }
        if path == 1 {
            return 7;
        }
        if path == 4 || path == 10 {
            return 8;
        }
        if path == 3 || path == 5 || path == 9 || path == 15 {
            return 9;
        }
    }
    0
}

pub fn get_path_cntp(paths: Vec<i8>) -> Vec<i8> {
    let mut cntp = vec![0; 10];
    for path in paths {
        cntp[path_p_type(path)] += 1;
    }
    cntp
}

pub fn get_path_cntodd(paths: Vec<i8>) -> i8 {
    let mut count = 0;
    for path in paths {
        if path & 1 == 1 {
            count += 1
        }
    }
    count
}

pub fn get_path_s(paths: Vec<i8>) -> HashSet<i8> {
    let mut s = HashSet::new();
    let cntp = get_path_cntp(paths);
    for i in 0..=7 {
        if cntp[i] & 1 == 1 {
            s.insert(i as i8);
        }
    }
    s
}

pub fn cycle_p_type(cycle: i8) -> usize {
    if cycle >= 19 && (cycle % 6 == 0 || cycle % 6 == 1 || cycle % 6 == 3) {
        return (cycle % 6) as usize;
    }
    if (cycle >= 19 && cycle % 6 == 2) || cycle == 14 {
        return 2;
    }
    if (cycle >= 16 && cycle % 6 == 4) || cycle == 16 {
        return 4;
    }
    if (cycle >= 16 && cycle % 6 == 5) || (cycle == 11 || cycle == 17) {
        return 5;
    }
    if cycle == 6 || cycle == 12 || cycle == 18 {
        return 6;
    }
    if cycle == 1 || cycle == 7 || cycle == 13 {
        return 7;
    }
    if cycle == 2 || cycle == 8 {
        return 8;
    }
    if cycle == 3 {
        return 9;
    }
    if cycle == 4 || cycle == 10 {
        return 10;
    }
    if cycle == 5 {
        return 11;
    }
    12
}

pub fn get_cycle_cntp(cycles: Vec<i8>) -> Vec<i8> {
    let mut cntp = vec![0; 13];
    for cycle in cycles {
        cntp[cycle_p_type(cycle)] += 1;
    }
    cntp
}

pub fn get_cycle_cntodd(cycles: Vec<i8>) -> i8 {
    let mut count = 0;
    for cycle in cycles {
        if cycle & 1 == 1 {
            count += 1
        }
    }
    count
}

pub fn get_cycle_s(cycles: Vec<i8>) -> HashSet<i8> {
    let mut s = HashSet::new();
    let cntp = get_cycle_cntp(cycles);
    for i in 0..=9 {
        if cntp[i] & 1 == 1 {
            s.insert(i as i8);
        }
    }
    s
}
