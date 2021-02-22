use std::collections::HashMap;
use std::collections::HashSet;


pub fn step(_l: &Vec<u32>) -> Vec<Vec<u32>>{
    let mut result: Vec<Vec<u32>> = vec![];
    let head = top(&_l);
    for &elem in _l{
        let mut temp: Vec<u32> = _l.clone();
        let b: Vec<u32> = vec![head+elem];
        temp.extend(b);
        result.extend(vec![temp]);
    }
    return result;
}

fn top (_l: &Vec<u32>) -> u32 {
    return _l[_l.len() - 1];
}


pub fn compute_sum<'a>(_n: u32, _memo: &'a mut HashMap<u32, u32>, _unseen: &mut HashSet<u32>) -> &'a mut HashMap<u32, u32> {
    let mut start :Vec<Vec<u32>> = vec![vec![1u32]];
    let mut _next: Vec<Vec<u32>> = vec![];
    let mut _idx: usize = 0;

    loop {
        let step_result = step(&start[_idx]);
        for _a in step_result {
            if _a[_a.len() - 1] == _n {
                mark(_memo, _n, (_a.len() - 1) as u32, _unseen);
                if done(_unseen){
                    return _memo;
                }
            }else if _a[_a.len() - 1] < _n{
                mark(_memo, _a[_a.len() - 1], (_a.len() - 1) as u32, _unseen);
                if done(_unseen){
                    return _memo;
                }
                _next.extend(vec![_a]);
            }
        }
        _idx += 1;
        if _idx == start.len() {
            start = _next.clone();
            _next = vec![];
            _idx = 0;
        }
    }
}

fn mark(_memo: &mut HashMap<u32, u32>, _key: u32, _val: u32, _unseen: &mut HashSet<u32>){
    if _unseen.contains(&_key){
        _unseen.remove(&_key);
        if !_memo.contains_key(&_key) {
            _memo.insert(_key, _val);
        }
    }
}

fn done(_unseen: &mut HashSet<u32>) -> bool {
    return _unseen.is_empty();
}