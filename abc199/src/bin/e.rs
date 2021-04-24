fn main() {
  proconio::input! {
    n: usize, m: usize,
    cs: [(usize, usize, usize); m],
  };
  let nums: Vec<usize> = (1..=n).collect();
  use itertools::Itertools;
  let mut ans: u64 = 0;
  'mainnums: for ns in nums.into_iter().permutations(n) {
    for &(x, y, z) in cs.iter() {
      let mut cnt = 0;
      for n in ns.iter().take(x) {
        if *n <= y {
          cnt += 1;
          if z < cnt {
            continue 'mainnums;
          }
        }
      }
    }
    ans += 1;
  }
  println!("{}", ans);
}
