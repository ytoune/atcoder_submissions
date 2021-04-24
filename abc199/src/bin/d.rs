fn main() {
  use proconio::marker::Usize1;
  proconio::input! {
    n: usize, m: usize,
    edges: [(Usize1, Usize1); m],
  };
  let ans = solve(n, edges);
  println!("{}", ans);
}

fn solve(n: usize, edges: Vec<(usize, usize)>) -> u128 {
  #[derive(Clone, Debug)]
  struct DotColor {
    pub color: Option<usize>, // 0, 1, 2
    pub fixed: bool,
    pub not: [bool; 3],
  }
  impl DotColor {
    fn new() -> DotColor {
      DotColor {
        color: None,
        fixed: false,
        not: [false; 3],
      }
    }
    fn count_possibility(&self) -> (u128, Option<usize>) {
      let mut a = 0;
      let mut r: Option<usize> = None;
      for ci in 0..3 {
        if !self.not[ci] {
          a += 1;
          if r.is_none() {
            r = Some(ci);
          }
        }
      }
      (a, r)
    }
  }
  impl Default for DotColor {
    fn default() -> DotColor {
      DotColor::new()
    }
  }

  // use petgraph::graph::UnGraph;
  // let mut graph = UnGraph::from_edges(edges.iter());
  let edges = {
    let mut e: Vec<Vec<usize>> = vec![vec![]; n];
    for (a, b) in edges {
      e[a].push(b);
      e[b].push(a);
    }
    e
  };
  use std::collections::*;
  let mut done: HashSet<(usize, usize)> = HashSet::new();
  let mut ans: u128 = 1;
  let mut colors = vec![DotColor::new(); n];
  let mut que: VecDeque<(usize, Option<(usize, usize)>)> = VecDeque::new();
  fn edge(a: usize, b: usize) -> (usize, usize) {
    if a <= b {
      (a, b)
    } else {
      (b, a)
    }
  }
  'main: loop {
    if let Some((i, prev)) = que.pop_back() {
      let c = &mut colors[i];
      if c.fixed {
        continue 'main;
      }
      if let Some((p, ci)) = prev {
        let e = edge(i, p);
        if done.contains(&e) {
          continue 'main;
        }
        c.not[ci] = true;
        done.insert(e);
      } else {
        c.fixed = true;
      }
      let (_a, ci) = c.count_possibility();
      if let Some(ci) = ci {
        c.color = Some(ci);
        for j in edges[i].iter().copied() {
          que.push_back((j, Some((i, ci))));
        }
        continue 'main;
      }
      return 0;
    } else {
      for (i, c) in colors.iter().enumerate() {
        if c.color.is_none() {
          que.push_back((i, None));
          continue 'main;
        }
      }
      println!("{:?}", colors);
      for c in colors.iter() {
        let (a, _) = c.count_possibility();
        ans *= a;
      }
      return ans;
    }
  }
}
