#[proconio::fastout]
fn main() {
  proconio::input! {
    n: usize,
    reqs: [(usize, usize, usize); n],
  };
  for &(x, y, r) in reqs.iter() {
    let mut p = (10000, 10000);
    for &(xt, yt, _) in reqs.iter() {
      if x < xt && y == yt && (p.0 > xt || p.1 > yt) {
        p = (xt, yt);
      }
    }
    let x2 = (x + r).min(p.0);
    let y2 = y + 1;
    println!("{} {} {} {}", x, y, x2, y2);
  }
}
