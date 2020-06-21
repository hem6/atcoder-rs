#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [i64;n],
        q: usize,
        bc: [(i64, i64);q],
    }

    let mut hm: HashMap<i64, i64> = HashMap::new();
    let mut sum = 0;
    for i in a {
        sum += i;
        *hm.entry(i).or_insert(0) += 1;
    }

    for (b, c) in bc {
        if let Some(&count) = hm.get(&b) {
            sum = sum + (c - b) * count;
            *hm.entry(c).or_insert(0) += count;
            hm.insert(b, 0);
        }

        println!("{}", sum);
    }
}
