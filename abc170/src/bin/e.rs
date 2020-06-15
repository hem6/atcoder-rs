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

    let mut q: Vec<BinaryHeap<u64>> = vec![BinaryHeap::new(); 200_000];
    let mut delq: Vec<BinaryHeap<u64>> = vec![BinaryHeap::new(); 200_000];

    let mut strongest: BinaryHeap<Reverse<u64>> = BinaryHeap::new();
    let mut del_strongest: BinaryHeap<Reverse<u64>> = BinaryHeap::new();

    let mut scores: Vec<u64> = Vec::new();
    let mut places: Vec<usize> = Vec::new();

    for (a, b) in ab {
        scores.push(a);
        places.push(b);
        q[b].push(a);
    }

    for que in &q {
        if let Some(&x) = que.peek() {
            strongest.push(Reverse(x));
        }
    }

    for (c, d) in cd {
        let cur = places[c];
        let score = scores[c];

        // update the place
        places[c] = d;

        update_que(&mut q[cur], &mut delq[cur]);
        update_que(&mut q[d], &mut delq[d]);

        // remove the child from the current place
        match q[cur].peek() {
            Some(&x) if x == score => {
                q[cur].pop();
                update_que(&mut q[cur], &mut delq[cur]);

                del_strongest.push(Reverse(score));
                if let Some(&y) = q[cur].peek() {
                    strongest.push(Reverse(y));
                }
            }
            _ => {
                delq[cur].push(score);
            }
        }

        // add the child to the next place
        match q[d].peek() {
            Some(&x) if x < score => {
                del_strongest.push(Reverse(x));
                strongest.push(Reverse(score));
                q[d].push(score);
            }
            None => {
                strongest.push(Reverse(score));
                q[d].push(score);
            }
            _ => {
                q[d].push(score);
            }
        }

        update_que(&mut strongest, &mut del_strongest);
        println!("{}", strongest.peek().unwrap().0);
    }
}

fn update_que<T: Ord + std::fmt::Debug>(q: &mut BinaryHeap<T>, delq: &mut BinaryHeap<T>) {
    loop {
        if q.peek() == None || delq.peek() == None || q.peek() != delq.peek() {
            return;
        }
        q.pop();
        delq.pop();
    }
}
