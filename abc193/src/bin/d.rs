fn main() {
  use proconio::marker::Chars;
  proconio::input! {
    k: usize,
    s: Chars,
    t: Chars,
  }
  struct Maker {
    used: [usize; 10],
  }
  impl Maker {
    fn make_hands(&mut self, chars: &[char]) -> [usize; 10] {
      let mut rt = [0; 10];
      for c in chars.iter().take(4) {
        let n = c.to_digit(10).unwrap() as usize;
        self.used[n] += 1;
        rt[n] += 1;
      }
      rt
    }
    fn get_used(self) -> [usize; 10] {
      self.used
    }
  }
  fn calc(hd: &[usize; 10], h: usize) -> usize {
    (1..=9)
      .map(|n| n * 10usize.pow((hd[n] + if h == n { 1 } else { 0 }) as u32))
      .sum()
  }
  let mut mk = Maker { used: [0; 10] };
  let s_hd = mk.make_hands(&s);
  let t_hd = mk.make_hands(&t);
  let used = mk.get_used();
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
