fn main() {
  use proconio::marker::Usize1;
  proconio::input! {
    n: usize,
    p: [Usize1; n-1],
    queries: [(Usize1, usize)],
  }
  let childrens = {
    let mut childrens = vec![vec![]; n];
    for (i, &pi) in p.iter().enumerate() {
      let i = i + 1;
      childrens[pi].push(i);
    }
    childrens
  };
  let mut descendants = vec![vec![]; n];
  for (i, &pi) in p.iter().enumerate() {
    let i = i + 1;
    let mut pi = pi;
    loop {
      descendants[pi].push(i);
      if pi == 0 {
        break;
      }
      pi = p[pi - 1];
    }
  }
  let mut heights = vec![0; n];
  let mut que = std::collections::VecDeque::new();
  que.push_front((0, 1));
  while let Some((i, h)) = que.pop_back() {
    heights[i] = h;
    for &j in childrens[i].iter() {
      que.push_front((j, h + 1));
    }
  }
  for (c, d) in queries.into_iter() {
    let mut ans = 0;
    for &u in descendants[c].iter() {
      if d == heights[u] {
        ans += 1;
      }
    }
    println!("{}", ans);
  }
}
