use proconio::input;

fn main() {
    input! {
        a:[[i32;3];3],
        n: usize,
        b: [i32;n]
    }

    if b.contains(&a[0][0]) && b.contains(&a[0][1]) && b.contains(&a[0][2])
        || b.contains(&a[1][0]) && b.contains(&a[1][1]) && b.contains(&a[1][2])
        || b.contains(&a[2][0]) && b.contains(&a[2][1]) && b.contains(&a[2][2])
        || b.contains(&a[0][0]) && b.contains(&a[1][0]) && b.contains(&a[2][0])
        || b.contains(&a[0][1]) && b.contains(&a[1][1]) && b.contains(&a[2][1])
        || b.contains(&a[0][2]) && b.contains(&a[1][2]) && b.contains(&a[2][2])
        || b.contains(&a[0][0]) && b.contains(&a[1][1]) && b.contains(&a[2][2])
        || b.contains(&a[0][2]) && b.contains(&a[1][1]) && b.contains(&a[2][0])
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
