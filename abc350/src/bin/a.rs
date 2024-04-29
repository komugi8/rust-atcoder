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
        s: String,
    }
    let s = s.as_str();
    if s >= "ABC001" && s <= "ABC349" && s != "ABC316"{
        println!("Yes");
    } else {
        println!("No");
    }
}
