fn main() {
  proconio::input! {
    n: usize, d: f64, h: f64,
    shields: [(f64, f64); n],
  };
  let ans: f64 = shields
    .iter()
    .map(|&(sd, sh)| (sd * h - d * sh) / (sd - d))
    .fold(0., |a, p| a.max(p));
  println!("{}", ans);
}
