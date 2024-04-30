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
        a: [Bytes; n],
        b: [Bytes; n],
    }
    for i in 0..n {
        for j in 0..n {
            if a[i][j] != b[i][j] {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}
