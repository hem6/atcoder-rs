#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut map1: HashMap<char, char> = HashMap::new();
    let mut map2: HashMap<char, char> = HashMap::new();

    let n = s.len();

    for i in 0..n {
        if map1.entry(s[i]).or_insert(t[i]) != &t[i] {
            println!("No");
            return;
        }
        if map2.entry(t[i]).or_insert(s[i]) != &s[i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
