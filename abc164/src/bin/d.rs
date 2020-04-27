#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        mut s: Chars,
    }

    s.reverse();

    let mut now: u32 = 0;
    let mut ten: u32 = 1;

    let mut map: HashMap<u32, u32> = HashMap::new();
    map.insert(0, 1);

    for c in s {
        now = (c.to_digit(10).unwrap() * ten + now) % 2019;
        map.entry(now).and_modify(|v| *v += 1).or_insert(1);
        ten = (ten * 10) % 2019;
    }

    let ans: u32 = map
        .into_iter()
        .map(|(_, v)| if v >= 2 { v * (v - 1) / 2 } else { 0 })
        .sum();

    println!("{}", ans);
}
