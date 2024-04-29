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
        mut s: Chars,
        t: Chars,
    }
    for s in s.iter_mut() {
        *s = s.to_ascii_uppercase();
    }
    let mut index = 0;
    let len = if t[2] == 'X' {2} else {3};
    for c in s {
        if c == t[index] {
            index += 1;
        }
        if index == len {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
