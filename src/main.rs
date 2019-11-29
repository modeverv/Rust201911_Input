fn main() {
    println!("Hello, world!");
    let i:i32 = r();
    println!("{}",i);
}
fn r<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}