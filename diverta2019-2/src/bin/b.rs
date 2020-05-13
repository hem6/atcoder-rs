#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        balls: [(isize, isize);n]
    }

    if n == 1 {
        println!("1");
        return;
    }

    let mut diffs: HashMap<(isize, isize), usize> = HashMap::new();

    for (&(x1, y1), &(x2, y2)) in balls.iter().tuple_combinations() {
        let mut diff = (x2 - x1, y2 - y1);
        if diff.0 < 0 || diff.0 == 0 && diff.1 < 0 {
            diff = (-diff.0, -diff.1);
        }

        *diffs.entry(diff).or_insert(0) += 1;
    }

    dbg!(&diffs);
    let max_count = diffs.values().max().unwrap();
    let ans = n - max_count;
    println!("{}", ans);
}
