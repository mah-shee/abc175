#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut ans = 0;
    for i in 0..3 {
        if s[i] == 'R' {
            ans += 1;
        }
    }
    if ans == 2 && s[1] == 'S' {
        ans = 1;
    }
    println!("{}", ans);
}
