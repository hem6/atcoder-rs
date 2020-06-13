#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut counter: Vec<usize> = vec![0; n];
    let mut colors = 0;
    let mut ans: usize = 1;

    for i in a {
        counter[i] += 1;
        if i == 0 {
            colors += 1;
            if colors > 3 {
                println!("0");
                return;
            }
            continue;
        }

        if counter[i - 1] < counter[i] {
            println!("0");
            return;
        }
        ans = ans * (counter[i - 1] - counter[i] + 1) % 1_000_000_007;
    }

    let permutation = (1..=3).rev().take(colors).fold(1, |acc, i| acc * i);
    println!("{}", ans * permutation % 1_000_000_007);
}
