use std::fs::File;
use std::path::Path;
use std::io::{Write,BufRead, BufReader, Error, ErrorKind};


fn main() {
    println!("Hello, world!");
    let i:i32 = r();
    println!("{}",i);
    let result = hello_to_file("/Users/seijiro/Downloads/a.txt");
}
fn r<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn hello_to_file(path: impl AsRef<Path>) -> Result<(),Error> {
    let mut file = File::create(path.as_ref())?;
    write!(file,"Hello,File")?;
    Ok(())
}