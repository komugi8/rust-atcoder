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
        s: Bytes,
    }
    let mut map = Map::new();
    for c in s {
        *map.entry(c).or_insert(0) += 1;
    }
    let mut cnt = Map::new();
    for v in map.values() {
        *cnt.entry(*v).or_insert(0) += 1;
    }
    let ans = if cnt.values().all(|v| *v == 2) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
