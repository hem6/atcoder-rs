#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        h: usize,
        w: usize,
        map:[Chars;h]
    }

    let mut alphabet_count: HashMap<char, i64> = HashMap::new();

    for i in 0..h {
        for j in 0..w {
            let c = map[i][j];
            *alphabet_count.entry(c).or_insert(0) += 1;
        }
    }

    if alphabet_count.len() > (h + 1) / 2 * (w + 1) / 2 {
        println!("No");
        return;
    }

    let mut count: HashMap<i64, i64> = HashMap::new();

    for (_, mut v) in alphabet_count {
        while v >= 4 {
            *count.entry(4).or_insert(0) += 1;
            v -= 4;
        }
        *count.entry(v).or_insert(0) += 1;
    }

    let requied_4 = (h as i64 / 2) * (w as i64 / 2);
    let n = count.get(&4).unwrap_or(&0);
    if *n < requied_4 {
        println!("No");
        return;
    }

    if h % 2 == 0 && w % 2 == 0 {
        println!("Yes");
        return;
    }

    *count.entry(2).or_insert(0) += (n - requied_4) * 2;

    if h % 2 + w % 2 == 1 {
        let requied_2 = if h % 2 == 0 { h / 2 } else { w / 2 };
        let &n = count.get(&2).unwrap_or(&0);
        if n == requied_2 as i64 {
            println!("Yes");
            return;
        }
    } else if h % 2 == 1 && w % 2 == 1 {
        let requied_2 = (h + 1) / 2 + (w + 1) / 2 - 2;
        if let Some(n) = count.get(&3) {
            *count.entry(2).or_insert(0) += 1;
            *count.entry(1).or_insert(0) += 1;
        }
        let &n = count.get(&2).unwrap_or(&0);
        if n == requied_2 as i64 && count.get(&1) == Some(&1) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
