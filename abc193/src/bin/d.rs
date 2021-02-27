fn main() {
  use proconio::marker::Chars;
  proconio::input! {
    k: usize,
    s: Chars,
    t: Chars,
  }
  let s: Vec<usize> = s
    .iter()
    .take(4)
    .map(|c| c.to_digit(10).unwrap() as usize)
    .collect();
  let t: Vec<usize> = t
    .iter()
    .take(4)
    .map(|c| c.to_digit(10).unwrap() as usize)
    .collect();
  fn make(ar: &[usize]) -> [usize; 10] {
    let mut rt = [0; 10];
    for &n in ar {
      rt[n] += 1;
    }
    rt
  }
  fn calc(hd: &[usize; 10], h: usize) -> usize {
    (1..=9)
      .map(|n| n * 10usize.pow((hd[n] + if h == n { 1 } else { 0 }) as u32))
      .sum()
  }
  let s_hd = make(&s);
  let t_hd = make(&t);
  let mut used: Vec<usize> = vec![0; 10];
  for n in s {
    used[n] += 1;
  }
  for n in t {
    used[n] += 1;
  }
  let mut win = 0;
  let mut lose = 0;
  for i in 1..=9 {
    for j in 1..=9 {
      let d = if i == j { 2 } else { 1 };
      if used[i] + d > k || used[j] + d > k {
        continue;
      }
      let cnt = if i == j {
        (k - used[i]) * (k - used[j] - 1)
      } else {
        (k - used[i]) * (k - used[j])
      };
      if calc(&s_hd, i) > calc(&t_hd, j) {
        win += cnt;
      } else {
        lose += cnt;
      }
    }
  }
  println!("{}", (win as f64) / ((win + lose) as f64));
}
