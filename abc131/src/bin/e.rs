#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    if k > comb(n - 1, 2) {
        println!("-1");
        return;
    }

    println!("{}", n - 1 + comb(n - 1, 2) - k);

    for i in 2..=n {
        println!("1 {}", i);
    }

    let mut count = comb(n - 1, 2);

    for i in 2..n {
        for j in i + 1..=n {
            if count == k {
                return;
            }
            count -= 1;

            println!("{} {}", i, j);
        }
    }
}

fn comb(n: usize, k: usize) -> usize {
    if n < k {
        return 0;
    }

    let mut a = 1;
    let mut b = 1;

    for i in 0..k {
        a *= n - i;
        b *= i + 1;
    }

    a / b
}
