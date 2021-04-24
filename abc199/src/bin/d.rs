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
  use petgraph::unionfind::UnionFind;
  let mut uf = UnionFind::<usize>::new(n);
  for (a, b) in edges.iter().copied() {
    uf.union(a, b);
  }

  use petgraph::graph::UnGraph;
  use petgraph::{Graph, Undirected};
  type Gr = Graph<(), (), Undirected, usize>;
  let gr: Gr = UnGraph::<(), (), usize>::from_edges(&edges);

  use std::collections::*;
  let mut grs = HashMap::<usize, (usize, usize)>::new();
  for n in 0..n {
    let l = uf.find_mut(n);
    grs
      .entry(l)
      .and_modify(|u| {
        u.1 += 1;
      })
      .or_insert((n, 1));
  }

  let mut ans = 1;

  for (_, (i, size)) in grs {
    if size == 1 {
      ans *= 3;
      continue;
    }
    use petgraph::visit::Dfs;
    let mut nodes = vec![];
    let mut dfs = Dfs::new(&gr, i.into());
    while let Some(n) = dfs.next(&gr) {
      nodes.push(n.index());
    }
    fn bfs(gr: &Gr, cur: usize, nodes: &[usize], colors: &mut [i32]) -> u128 {
      if cur == nodes.len() {
        return 1;
      }
      let mut res = 0;
      let i = nodes[cur];
      'color: for c in 1..=3 {
        for j in gr.neighbors(i.into()) {
          let ji = j.index();
          if colors[ji] != 0 && colors[ji] == c {
            continue 'color;
          }
        }
        colors[i] = c;
        res += bfs(gr, cur + 1, nodes, colors);
        colors[i] = 0;
      }
      res
    }
    let mut colors = vec![0; n];
    colors[nodes[0]] = 1;
    ans *= 3 * bfs(&gr, 1, &nodes, &mut colors);
  }

  ans
}
