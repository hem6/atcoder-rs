use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        f: [[Usize1;2];m],
        b: [[Usize1;2];k],
    }

    let mut friend_graph: Vec<Vec<usize>> = vec![vec![]; n];

    for v in f {
        let (from, to) = (v[0], v[1]);
        friend_graph[from].push(to);
        friend_graph[to].push(from);
    }

    let mut block_graph: Vec<Vec<usize>> = vec![vec![]; n];

    for v in b {
        let (from, to) = (v[0], v[1]);
        block_graph[from].push(to);
        block_graph[to].push(from);
    }

    let mut cluster: Vec<i32> = vec![-1; n];
    let mut cluster_no = 0;

    for i in 0..n {
        if cluster[i] != -1 {
            continue;
        }

        dfs(i, &friend_graph, &mut cluster, cluster_no);
        cluster_no += 1;
    }

    let mut cluster_member_count = vec![0; cluster_no as usize];
    for i in 0..n {
        cluster_member_count[cluster[i] as usize] += 1;
    }

    for i in 0..n {
        let total = cluster_member_count[cluster[i] as usize];
        let direct_friend = friend_graph[i].len();
        let blocked = block_graph[i]
            .iter()
            .filter(|&v| cluster[*v as usize] == cluster[i])
            .count();
        let ans = total - direct_friend - blocked - 1;
        println!("{}", ans);
    }
}

fn dfs(i: usize, g: &Vec<Vec<usize>>, cluster: &mut Vec<i32>, cluster_no: i32) {
    cluster[i] = cluster_no;
    // なんらかの処理(行きがけ順)
    for &next in &g[i] {
        if cluster[next] != -1 {
            continue;
        }
        dfs(next, g, cluster, cluster_no);
    }
}
