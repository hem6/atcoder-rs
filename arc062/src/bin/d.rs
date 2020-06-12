#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        s:Chars,
    }

    let mut ans = 0;
    let mut g = true;

    for c in s {
        let mine = if g { 'g' } else { 'p' };
        if c == 'g' && mine == 'p' {
            ans += 1;
        } else if c == 'p' && mine == 'g' {
            ans -= 1;
        }
        g = !g;
    }

    println!("{}", ans);
}
