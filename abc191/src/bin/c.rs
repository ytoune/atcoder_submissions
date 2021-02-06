use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    h: usize,
    w: usize,
    s: [Chars; h],
  };
  let dirs = [(0, 0), (0, 1), (1, 0), (1, 1)];

  let mut res = 0;

  for i in 0..h - 1 {
    for j in 0..w - 1 {
      match dirs
        .iter()
        .map(|(h, w)| if s[i + h][j + w] == '#' { 1 } else { 0 })
        .sum()
      {
        1 | 3 => {
          res += 1;
        }
        _ => {}
      }
    }
  }

  println!("{}", res);
}
