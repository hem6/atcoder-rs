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
        g: [[Usize1;2];n-1],
        p: [[Usize1;2];q],
    }

    let commands = p.iter().map(|v| (v[0], v[1])).into_group_map();
    let commands: HashMap<usize, i32> = commands
        .iter()
        .map(|(k, v)| (*k, v.iter().map(|i| (*i + 1) as i32).sum::<i32>()))
        .collect();

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];

    for v in g {
        let (from, to) = (v[0], v[1]);
        graph[from].push(to);
        graph[to].push(from);
    }

    let mut ans: Vec<i32> = vec![-1; n];

    dfs(0, &graph, &commands, 0, &mut ans);

    for i in ans {
        println!("{}", i);
    }
}

fn dfs(
    i: usize,
    g: &Vec<Vec<usize>>,
    commands: &HashMap<usize, i32>,
    add: i32,
    ans: &mut Vec<i32>,
) {
    let v = (add + commands.get(&i).unwrap_or(&0)) as i32;
    ans[i] = v;
    for next in &g[i] {
        if ans[*next] != -1 {
            continue;
        }
        dfs(*next, g, commands, v, ans);
    }
}
