#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [u64;n],
    }

    if a.contains(&0) {
        println!("0");
        return;
    }

    let mut ans = 1u64;
    for i in a {
        let res = ans.checked_mul(i);
        if let Some(v) = res {
            ans = v;
        } else {
            println!("-1");
            return;
        }
    }

    if ans > 10u64.pow(18) {
        println!("-1");
        return;
    }

    println!("{}", ans);
}
