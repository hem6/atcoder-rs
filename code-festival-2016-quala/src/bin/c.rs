#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        s: Chars,
        mut k: usize,
    }

    let map: HashMap<char, usize> = (b'a'..=b'z')
        .map(char::from)
        .zip(std::iter::once(0).chain((1..=25).rev()))
        .collect();

    let num_to_a: Vec<usize> = s.iter().map(|&c| *map.get(&c).unwrap()).collect();
    let mut ans_s = s.clone();

    for i in 0..s.len() {
        if k < num_to_a[i] {
            continue;
        }
        ans_s[i] = 'a';
        k -= num_to_a[i];
    }

    let last = ans_s.len() - 1;
    ans_s[last] = ((ans_s[last] as u8 - b'a' + (k % 26) as u8) % 26 + b'a') as char;

    for c in ans_s {
        print!("{}", c);
    }
}
