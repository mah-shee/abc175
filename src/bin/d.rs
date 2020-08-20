#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
        c: [isize; n]
    }
    let mut ans: isize = -1_000_000_000;
    for i in 0..n {
        let mut next = p[i] - 1;
        let mut res = vec![0; k + 1];
        for j in 1..=k {
            res[j] = res[j - 1] + c[next];
            next = p[next] - 1;
        }
        res.remove(0);
        res.sort();
        ans = max(ans, res[k - 1]);
    }

    println!("{}", ans);
}
