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
        q: usize,
        mut t: [usize; q],
    }
    for i in 0..q {
        t[i] -= 1;
    }
    let mut tooth = vec![true; n];
    for i in 0..q {
        if tooth[t[i]] {
            tooth[t[i]] = false;
        } else {
            tooth[t[i]] = true;
        }
    }
    let mut cnt = 0;
    for i in 0..n {
        if tooth[i] {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
