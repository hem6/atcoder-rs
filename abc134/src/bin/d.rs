#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [i32;n],
    }

    let mut ans: Vec<i32> = vec![0; n + 1];

    for i in 0..n {
        let i = n - i;
        let mut j = i;
        let mut count = 0;

        while j <= n {
            if ans[j] == 1 {
                count += 1;
            }
            j += i;
        }

        if a[i - 1] == count % 2 {
            ans[i] = 0;
        } else {
            ans[i] = 1;
        }
    }

    let total: i32 = ans.iter().sum();
    println!("{}", total);

    let res = ans
        .iter()
        .enumerate()
        .filter(|&(i, v)| *v == 1)
        .map(|(i, v)| i)
        .join(" ");

    println!("{}", res);
}
