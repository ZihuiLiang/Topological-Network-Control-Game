/*!The linearforest module contains the data structures and algorithms for solving topological network control games in linear forests.*/

use std::collections::HashMap;
use std::cmp::{Ordering};
use std::fmt;
#[derive(Clone, Eq, Hash)]

/** `Path` stores the unclaimed parts of a path during a play. 
 */
pub struct Path {
    p: Vec<i32>, //If is_color=true, then p.len()=2, p[0] is the number of unclaimed vertices in the left part and p[1] the number of unclaimed vertices in the right part. Otherwise, is_color=false, p.len()=2, p[0] is the vertex number of the path and p[1]=0.
    is_colored: bool, // If the path is selected, then is_color=true.
}

impl Path {
    /** Create a new Path. Note that p.len() should be 2*/
    pub fn new(p: &Vec<i32>, is_colored: bool) -> Self {
        assert_eq!(2, p.len());
        Path {p: (*p).clone(), is_colored: is_colored}
    }
    /** Return the size of unclaimed vertices*/
    fn size(&self) -> i32 { //
        self.p[0] + self.p[1]
    }
    /** Return a minimum description (p[0]>=p[1]) of the path.*/
    fn minimum_describ(&mut self) {
        if self.p[0] < self.p[1] {
            self.p.swap(0, 1);
        }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_colored {
            return write!(f, "({}, {})", self.p[0], self.p[1]);
        }
        write!(f, "{}", self.p[0])
    }
}

/** `PartialOrd` define the partial ordering of `Path`.*/

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/** `Ord` define the ordering of `Path`.*/

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_colored == other.is_colored {
            return self.p.cmp(&other.p);
        }
        self.is_colored.cmp(&other.is_colored).reverse()
    }
}

/** `PartialEq::eq` define the equal ordering of `Path`.*/

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p && self.is_colored == other.is_colored
    }
}

/** `Configuration` stores the configuration of linear forest by a vector of `Path`s.*/

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Configuration {
    c: Vec<Path>
}   



impl Configuration {
    /** Create a configuration of linear forest.*/
    pub fn new(c: &Vec<Path>) -> Self {
        Configuration { c: (*c).clone()}
    }

    /** Make the configuration into minimum description by sorting the `Path`s according to the ordering of `Path`. */

    pub fn minimum_describ(&mut self) {
        for i in &mut self.c {
            i.minimum_describ();
        }
        self.c.sort();
    }

    /** Make the configuration into minimum description and remove the empty `Path`s. */

    pub fn clean(&mut self) {
        self.minimum_describ();
        self.c = self.c.clone().into_iter().filter(|x| x.p[0] > 0 || x.p[1] > 0).collect();
    }

    /** Return the number of unclaimed vertices. */

    pub fn size(&self) -> i32 {
        let mut sz = 0i32;
        for i in & self.c {
            sz += i.size();
        }
        sz
    }

    /** If the configuration contains claimed vertices, return false. Otherwise return true.*/

    pub fn is_empty(&self) -> bool {
        self.c.is_empty()
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, p) in self.c.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", p)?;
        }
        write!(f,"]")?;
        Ok(())
    }
}

/** `NextConfigGen` stores the required elements for generating all the next moves of a particular configuration step by step. */
struct NextConfigGen {
    c: Configuration,
    t: i32,
    selected_path_id: usize, //0~c.len()
    selected_path_part: usize, //0 for p.0, 1 for p.1
    last_move: i32,
}

impl NextConfigGen {
    /** Create a `NextConfigGen` with a configuration and the neighbourhood parameter $t$*/
    fn new(c: &Configuration, t: i32) -> Self {
        NextConfigGen {c: (*c).clone(), t: t, selected_path_id: 0, selected_path_part:0, last_move: -1}
    }

    /** If there is any ungenerated move then return true and `nextconfig` stores the next configuration. Otherwise return false.*/
    fn next(&mut self, nextconfig: &mut Configuration, movesize: &mut i32) -> bool {
        if self.c.is_empty() {
            return false;
        }
        if self.c.c[0].is_colored {
            self.last_move += 1;
            loop {
                if self.last_move == self.c.c[0].p[self.selected_path_part] ||  self.last_move > self.t {
                    self.last_move = 0;
                    self.selected_path_part += 1;
                    if self.selected_path_part > 1 {
                        return false;
                    }
                    continue;
                }
                *nextconfig = self.c.clone();
                nextconfig.c[0].p[self.selected_path_part] = std::cmp::max(self.c.c[0].p[self.selected_path_part] - self.last_move - self.t - 1, 0);
                *movesize = std::cmp::min(self.last_move, self.t) + std::cmp::min(self.c.c[0].p[self.selected_path_part] - self.last_move - 1, self.t) + 1;
                nextconfig.clean();
                return true;
            }
        } else {
            self.last_move += 1;
            loop {
                if self.selected_path_id >= self.c.c.len() {
                    return false;
                }
                let currentpath = self.c.c[self.selected_path_id].p[0];
                if self.last_move == currentpath {
                    self.last_move = 0;
                    self.selected_path_id += 1;
                    continue;
                }
                *nextconfig = self.c.clone();
                nextconfig.c[self.selected_path_id] = Path::new(& vec![std::cmp::max(self.last_move - self.t, 0), std::cmp::max(currentpath - self.last_move - self.t - 1, 0)],true);
                *movesize = std::cmp::min(self.last_move, self.t) + std::cmp::min(currentpath - self.last_move - 1, self.t) + 1;
                nextconfig.clean();
                return true;
            }
        }
    }
}


/** `Searcher` stores the required element for searching the optimal (if both the players want to maximize their scores) results of a configuration.*/
pub struct Searcher {
    opt:HashMap<Configuration,i32>, // the hash map storing the optimal results of searched configurations
    t: i32 // the neighbourhood parameter
}

impl Searcher {
    /** Create a new searcher with the neighbourhood parameter $t$. */
    pub fn new(t: i32)-> Self {
        Searcher{opt: HashMap::new(), t: t}
    }

    /** Search the result of a given configuration. */

    pub fn search(&mut self, mut config: Configuration) -> i32 {
        config.clean();
        if let Some(x) = self.opt.get(&config) {
            return *x;
        }
        let mut v = -config.size();
        let mut gen = NextConfigGen::new(&config, self.t);
        let mut nextconfig = Configuration::new(& vec![]);
        let mut movesize = 0i32;
        while gen.next(&mut nextconfig, &mut movesize) {
            v = std::cmp::max(v, movesize - self.search(nextconfig.clone()));
        }
        self.opt.insert((config).clone(), v);
        v
    }
}

/** A module of solving two-path linear forest by patterns. */
pub mod two_path_pattern_solver {
    use crate::linearforest::Configuration;
    /** Determine if it's a draw configuration by patterns. */
    pub fn is_draw(mut config: Configuration) -> bool {
        config.clean();
        assert_eq!(config.c.len(), 2);
        assert_eq!(config.c[0].is_colored, false);
        if config.c[0].p[0] == 5 || config.c[1].p[0] == 5 {
            return false;
        }
        
        (vec![config.c[0].p[0], config.c[1].p[0]] == vec![1, 1]) || // 1 1
        (vec![config.c[0].p[0], config.c[1].p[0]] == vec![2, 2]) || // 2 2
        (vec![config.c[0].p[0], config.c[1].p[0]] == vec![7, 7]) || // 7 7
        (config.c[0].p[0] == 2 && config.c[1].p[0] % 6 == 0) ||     // 2 6x
        (config.c[0].p[0] % 6 == 0 && config.c[1].p[0] == 2) ||     // 6x 2
        (config.c[0].p[0] == 7 && config.c[1].p[0] % 6 == 5) ||     // 7 6x+5 
        (config.c[0].p[0] % 6 == 5 && config.c[1].p[0] == 7) ||     // 6x+5 7
        (config.c[0].p[0] % 6 == 0 && config.c[1].p[0] % 6 == 0) || // 6x 6y
        (config.c[0].p[0] % 6 == 5 && config.c[1].p[0] % 6 == 5)    // 6x+5 6y+5
    }
}