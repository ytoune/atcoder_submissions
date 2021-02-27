fn main() {
  proconio::input! {
    n: usize,
    s: [(u64, u64, u64); n],
  };
  let mut minv = None;
  for (a, p, x) in s {
    if a < x {
      minv = match minv {
        None => Some(p),
        Some(q) => Some(if q < p { q } else { p }),
      }
    }
  }
  if let Some(p) = minv {
    println!("{}", p);
  } else {
    println!("-1");
  }
}
