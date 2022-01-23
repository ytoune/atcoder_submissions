fn main() {
  unimplemented!();
  // use proconio::marker::Usize1;
  // proconio::input! {
  //   n: usize, t: usize, l: u64,
  //   vicinities: [(Usize1, Usize1); t],
  // };
  // use std::collections::HashSet;
  // use std::collections::VecDeque;
  // let mut lens = vec![HashSet::new(); n];
  // lens[0].insert(0);
  // let mut tree = vec![HashSet::new(); n];
  // let mut ans = vec![None; n];
  // for (t, (i, j)) in vicinities.into_iter().enumerate() {
  //   let t = t + 1;
  //   tree[i].insert(j);
  //   if let Some(ln) = lens[i] {
  //     let mut que = VecDeque::new();
  //     struct Tsk {
  //       i: usize,
  //       ln: u64,
  //     };
  //     que.push_back(Tsk { i, ln });
  //     while let Some(tsk) = que.pop_front() {
  //       let ln = tsk.ln + 1;
  //       if let Some(vs) = tree.get(tsk.i) {
  //         for &v in vs.iter() {
  //           if let Some(ln2) = lens[v] {
  //             if ln2 > ln {
  //               lens[v] = Some(ln);
  //               que.push_back(value: T)
  //             }
  //           }
  //         }
  //       }
  //     }
  //   }
  // }
}
