#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        l: [usize; n],
    }
    let mut ans = 0;
    if n <= 2 {
        println!("{}", ans);
        return;
    }
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            if l[i] == l[j] {
                continue;
            }
            for k in j + 1..n {
                if l[i] == l[k] || l[j] == l[k] {
                    continue;
                }
                if l[i] + l[j] > l[k] && l[j] + l[k] > l[i] && l[i] + l[k] > l[j] {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
