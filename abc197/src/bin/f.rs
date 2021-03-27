fn main() {
  use proconio::marker::Usize1;
  proconio::input! {
    n: usize,
    m: usize,
    chs: [(Usize1, Usize1, char); m],
  };
  let chs = {
    let mut chss = vec![vec![]; n];
    for (pa, pb, c) in chs.into_iter() {
      chss[pa].push((pb, c));
      chss[pb].push((pa, c));
    }
    chss
  };
  use std::collections::VecDeque;
  let mut que: VecDeque<(usize, Vec<char>)> = VecDeque::new();
  que.push_back((0, vec![]));
  let goal = n - 1;
  while let Some((pos, chars)) = que.pop_front() {
    if pos == goal {
      let len = chars.len();
      let mut f = true;
      for (i, c) in chars.iter().enumerate() {
        if chars[len - i - 1] != *c {
          f = false;
          break;
        }
      }
      if f {
        println!("{}", len);
        return;
      }
      if len > m * 2 {
        println!("-1");
        return;
      }
    }
    for (p, c) in chs[pos].iter() {
      let mut chars: Vec<char> = chars.iter().copied().collect();
      chars.push(*c);
      que.push_back((*p, chars));
    }
  }
}
