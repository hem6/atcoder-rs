#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [i64;n+1],
    }

    if n == 0 && a[0] == 1 {
        println!("1");
        return;
    } else if n == 0 {
        println!("-1");
        return;
    }

    let mut mx1 = vec![0; n + 2];
    for i in 0..n + 1 {
        mx1[i + 1] = mx1[i] + a[n - i];
    }
    mx1.reverse();

    let mut ans = 1;
    let mut bef = 1;

    for i in 1..n + 1 {
        let mx = (bef - a[i - 1]) * 2;
        let node_count = min(mx, mx1[i]);
        if node_count < a[i] {
            ans = -1;
            break;
        }
        ans += node_count;
        bef = node_count;
    }

    println!("{}", ans);
}
