#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut bc: [[usize;2];m],
    }

    bc.sort_by_key(|v| Reverse(v[1]));
    a.sort();

    let mut ans = 0;
    let mut cur = 0;

    for i in 0..m {
        if cur >= n {
            break;
        };
        for _ in 0..bc[i][0] {
            if cur >= n {
                break;
            };
            ans += max(a[cur], bc[i][1]);
            cur += 1;
        }
    }
    for i in cur..n {
        ans += a[i];
    }

    println!("{}", ans);
}
