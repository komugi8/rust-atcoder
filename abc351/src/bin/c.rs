#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use proconio::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(dead_code)]
type Map<K, V> = BTreeMap<K, V>;
#[allow(dead_code)]
type Set<T> = BTreeSet<T>;
#[allow(dead_code)]
type Deque<T> = VecDeque<T>;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut stk = Vec::new();
    for i in 0..n {
        stk.push(a[i]);
        let mut check = true;
        while check {
            check = false;
            if stk.len() <= 1 {
                continue;
            }
            if stk[stk.len() - 2] == stk[stk.len() - 1] {
                let top = stk.pop().unwrap();
                stk.pop();
                stk.push(top + 1);
                check = true;
            } else {
                continue;
            }
        }
    }
    println!("{}", stk.len())
}
