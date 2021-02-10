#[proconio::fastout]
fn main() {
  proconio::input! {
    n: usize,
    m: usize,
    t: usize,
    s: [(usize, usize); m],
  };
  let bsize = n;
  let mut n = n;
  let mut c = 0;
  for (a, b) in s {
    if n <= a - c {
      println!("No");
      return;
    }
    n -= a - c;
    n = (n + b - a).min(bsize);
    c = b;
  }
  if n <= t - c {
    println!("No");
    return;
  }
  println!("Yes");
}
