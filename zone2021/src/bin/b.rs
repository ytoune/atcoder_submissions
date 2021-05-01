fn main() {
  proconio::input! {
    n: usize, d: f64, h: f64,
    shields: [(f64, f64); n],
  };
  let ans: Option<f64> = shields
    .iter()
    .map(|&(sd, sh)| (sd * h - d * sh) / (sd - d))
    .fold(None, |a: Option<f64>, p: f64| {
      a.map(|a| if a < p { p } else { a }).or_else(|| Some(p))
    });
  if let Some(ans) = ans {
    println!("{}", ans.max(0.));
  }
}
