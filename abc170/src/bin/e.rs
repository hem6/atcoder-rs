#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(u64, Usize1);n],
        cd: [(Usize1, Usize1);q]
    }

    let mut q: Vec<BTreeMap<u64, u64>> = vec![BTreeMap::new(); 200_000];
    let mut strongest: BTreeMap<u64, u64> = BTreeMap::new();

    let mut scores: Vec<u64> = Vec::new();
    let mut places: Vec<usize> = Vec::new();

    for (a, b) in ab {
        scores.push(a);
        places.push(b);
        *q[b].entry(a).or_insert(0) += 1;
    }

    for que in &q {
        if let Some((&max_score, _)) = que.range(..).next_back() {
            *strongest.entry(max_score).or_insert(0) += 1;
        }
    }

    for (c, d) in cd {
        let cur = places[c];
        let score = scores[c];

        // update the place
        places[c] = d;

        // remove the child from the current place
        match q[cur].range(..).next_back() {
            Some((&max_score, _count)) if max_score == score => {
                remove_one(&mut q[cur], &score);
                remove_one(&mut strongest, &score);
                if let Some((&next_max_score, _count)) = q[cur].range(..).next_back() {
                    *strongest.entry(next_max_score).or_insert(0) += 1;
                }
            }
            _ => {
                remove_one(&mut q[cur], &score);
            }
        }

        // add the child to the next place
        match q[d].range(..).next_back() {
            Some((&max_score, _count)) if max_score < score => {
                remove_one(&mut strongest, &max_score);
                *strongest.entry(score).or_insert(0) += 1;
                *q[d].entry(score).or_insert(0) += 1;
            }
            None => {
                *strongest.entry(score).or_insert(0) += 1;
                *q[d].entry(score).or_insert(0) += 1;
            }
            _ => {
                *q[d].entry(score).or_insert(0) += 1;
            }
        }

        println!("{}", strongest.range(..).next().unwrap().0);
    }
}

fn remove_one(bm: &mut BTreeMap<u64, u64>, key: &u64) {
    *bm.get_mut(key).unwrap() -= 1;
    if bm.get(key) == Some(&0) {
        bm.remove(key);
    }
}
