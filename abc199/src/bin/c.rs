fn main() {
  use proconio::marker::Chars;
  proconio::input! {
    n: usize,
    mut s: Chars,
    q: usize,
    qs: [(u8, usize, usize); q],
  };
  let mut u = false;
  let idx = |i: usize, u: bool| -> usize {
    let i = i - 1;
    if u {
      if i < n {
        n + i
      } else {
        i - n
      }
    } else {
      i
    }
  };
  for (t, a, b) in qs {
    match (t, a, b) {
      (1, a, b) => {
        s.swap(idx(a, u), idx(b, u));
      }
      (2, 0, 0) => {
        u = !u;
      }
      _ => unreachable!(),
    }
  }
  if u {
    for i in 0..n {
      s.swap(i, i + n);
    }
  }
  println!("{}", s.into_iter().collect::<String>());
}
