use text_io::read;

fn main() {
    let n: usize = read!();
    let m: usize = read!();

    let mut bit_table: Vec<i32> = Vec::with_capacity(m);

    for _ in 0..m {
        let k: usize = read!();
        let mut b = 0;

        for _ in 0..k {
            let v: i32 = read!();
            b |= 1 << v - 1;
        }

        bit_table.push(b);
    }

    let p = (0..m).map(|_| read!()).collect::<Vec<u32>>();

    let mut ans = 0;
    for i in 0..(1 << n) {
        let mut ok_flag = true;

        for j in 0..m {
            if (bit_table[j] & i).count_ones() % 2 != p[j] {
                ok_flag = false;
                break;
            }
        }

        if ok_flag {
            ans += 1;
        }
    }

    println!("{}", ans);
}
