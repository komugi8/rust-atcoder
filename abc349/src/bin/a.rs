use proconio::input;

fn main() {
    input! {
        n: i32,
        a: [i32; n-1],
    }
    let s = a.iter().sum::<i32>();
    println!("{}", -s);
}
