fn main() {
  proconio::input! {
    n: usize,
    balls: [(i64, usize); n],
  };
  let balls = {
    use itertools::Itertools;
    let mut balls: Vec<_> = balls.into_iter().map(|(pos, color)| (color, pos)).collect();
    balls.sort();
    let balls: Vec<_> = balls
      .into_iter()
      .group_by(|q| q.0)
      .into_iter()
      .map(|(color, items)| {
        let mut min: Option<i64> = None;
        let mut max: Option<i64> = None;
        for (_, p) in items {
          min = min.map(|v| v.min(p)).or_else(|| Some(p));
          max = max.map(|v| v.max(p)).or_else(|| Some(p));
        }
        (color, min.unwrap(), max.unwrap())
      })
      .collect();
    balls
  };
  let mut dp = vec![vec![(0, 0); 2]; balls.len() + 1];
  for (i, b) in balls.iter().enumerate() {
    for (j, &(b1, b2)) in [(b.1, b.2), (b.2, b.1)].iter().enumerate() {
      dp[i + 1][j] = dp[i]
        .iter()
        .map(|d| (d.0 + (d.1 - b1).abs() + (b1 - b2).abs(), b2))
        .min()
        .unwrap();
    }
  }
  let min = dp.last().unwrap().iter().min().unwrap();
  println!("{}", min.0 + min.1.abs());
}
