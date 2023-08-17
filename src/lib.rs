/*! The lib packge is the denpendency of solve_linearforest binary package. It contains useful module linearforest.
    The following is an example: Use the searcher to search the result of finitely many 2-path linear forests and apply these results to verify the correctness of `two_path_pattern_solver`.
```
use solve_linearforest::{linearforest::*};
const MAX_SIZE:i32 = 100i32; // the maximum size of each path in the 2-path linear forest 
const T:i32 = 1i32; // the neighbourhood parameter


fn main() { ·
    let mut searcher = Searcher::new(T);
    let mut any_counter_case = false;
    for i in 1..MAX_SIZE { // enumerate
        for j in 1..MAX_SIZE {
            let config = Configuration::new(&vec![Path::new(&vec![i,0], false),Path::new(&vec![j,0], false)]);
            let search_is_draw = searcher.search(config.clone()) == 0; // Search the result of config
            if search_is_draw != two_path_pattern_solver::is_draw(config.clone()) { // If there exists any counter case then output the counter case.
                any_counter_case = true;
                println!("Counter case:{} {} {}", config, search_is_draw, two_path_pattern_solver::is_draw(config.clone()));
            }
        }
    }
    if any_counter_case {
        println!("Done! There exists some counter case!");
    } else {
        println!("Done! No counter case!");
    }
}


```

*/
pub mod linearforest;