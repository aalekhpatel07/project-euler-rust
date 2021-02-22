use std::collections::{HashMap, HashSet};

mod compute;


fn find_best_sum(_n: u32) -> u32{

    let mut memo: HashMap<u32, u32> = HashMap::new();
    let mut _unseen: HashSet<u32> = HashSet::new();

    for i in 2.._n+1{
        _unseen.insert(i);
    }

    return match _n {
        0 => 0,
        1 => 0,
        2 => 1,
        3 => 3,
        4 => 5,
        elem => {compute::compute_sum(elem, &mut memo, &mut _unseen).values().sum()}
    }
}

fn solve(sz: u32){

    let mut _rng = vec![];
    for i in 1..sz+1 {
        _rng.push(i);
    }

    println!("Started computation.");

    let acc = find_best_sum(sz);

    println!("Finished computation.");
    println!("The sum of m(k) from 1 to {0} is: {1}", sz, acc);
}

pub fn main(target: u32){
    solve(target);
}