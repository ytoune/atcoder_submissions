fn main() {
  use proconio::marker::Chars;
  proconio::input! {
    n: usize,
    s: Chars,
    x: Chars,
  };
  let pairs = {
    let mut s: Vec<(usize, usize, char)> = s
      .into_iter()
      .zip(x.into_iter())
      .enumerate()
      .map(|(i, (c, x))| (i, c.to_digit(10).unwrap() as usize, x))
      .collect();
    s.reverse();
    s
  };
  // dp[i][r] = i ラウンドに 7 で割ったあまりが r なら高橋君の勝ちにできる (true)
  let mut dp = vec![[false; 7]; n + 1];
  // n ラウンドに 7 で割ったあまりが 0 なら高橋君の勝ちにできる
  dp[n][0] = true;
  for (i, s, x) in pairs {
    match x {
      'T' => {
        for r in 0..7 {
          if dp[i + 1][(10 * r) % 7] || dp[i + 1][(10 * r + s) % 7] {
            dp[i][r] = true;
          }
        }
      }
      'A' => {
        for r in 0..7 {
          if dp[i + 1][(10 * r) % 7] && dp[i + 1][(10 * r + s) % 7] {
            dp[i][r] = true;
          }
        }
      }
      _ => unreachable!(),
    };
  }
  // 最初（0 ラウンド）に 7 で割ったあまりが 0 なら高橋君の勝ちにできる
  if dp[0][0] {
    println!("Takahashi");
  } else {
    println!("Aoki");
  }
}
