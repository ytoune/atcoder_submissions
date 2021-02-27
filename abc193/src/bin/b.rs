fn main() {
  proconio::input! {
    n: usize,
    s: [(u64, u64, u64); n],
  };
  let minv = s
    .into_iter()
    .filter(|(a, _, x)| a < x)
    .fold(None, |v, (_, p, _)| {
      v.map(|q| if q < p { q } else { p }).or_else(|| Some(p))
    });
  if let Some(p) = minv {
    println!("{}", p);
  } else {
    println!("-1");
  }
}
