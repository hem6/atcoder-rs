#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        mut b: [Usize1; n]
    }

    let mut ans = Vec::new();

    for i in 0..n {
        let end = b.len();
        for j in 0..end {
            let j = end - j - 1;
            if b[j] == j {
                ans.push(j);
                b.remove(j);
                break;
            }
        }
    }

    if b.len() > 0 {
        println!("-1");
        return;
    }

    for i in ans.iter().rev() {
        println!("{}", i + 1);
    }
}
