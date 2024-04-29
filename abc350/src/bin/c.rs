#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use proconio::*;
#[allow(unused_imports)]
use std::collections::*;
use std::vec;

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
        mut a: [usize; n],
    }
    for i in 0..n {
        a[i] -= 1;
    }
    let mut pos = vec![0; n];
    for (i, v) in a.iter().enumerate() {
        pos[*v] = i;
    }
    let mut ans: Vec<Pair> = Vec::new();
    for i in 0..n - 1 {
        let idx = pos[i];
        if i == a[i] {
            continue;
        }
        ans.push(Pair(i + 1, idx + 1));
        pos[a[i]] = idx;
        pos[i] = i;
        a[idx] = a[i];
        a[i] = i;
    }
    println!("{}", ans.len());
    for i in ans {
        println!("{} {}", i.0, i.1);
    }
}

struct Pair(usize, usize);
