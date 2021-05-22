fn main() {
  proconio::input! { a: u128, b: u128, mut k: u128 };
  let mut counter = Counter {
    cache: HashMap::new(),
  };
  solve(a, b, k, "".into(), &mut counter);
}

fn solve(a: u128, b: u128, k: u128, word: String, counter: &mut Counter) {
  if k == 0 {
    println!("{}", word);
    return;
  }
  if 0 < a && 0 < b {
    let c = counter.count(a - 1, b);
    if k <= c {
      solve(a - 1, b, k, format!("{}a", word), counter);
    } else {
      solve(a, b - 1, k - c, format!("{}b", word), counter);
    }
    return;
  }
  let mut word = word;
  let mut a = a;
  while 0 < a {
    word = format!("{}a", word);
    a -= 1;
  }
  let mut b = b;
  while 0 < b {
    word = format!("{}b", word);
    b -= 1;
  }
  println!("{}", word);
}

use std::collections::*;
struct Counter {
  cache: HashMap<(u128, u128), u128>,
}
impl Counter {
  fn count(&mut self, a_num: u128, b_num: u128) -> u128 {
    let sum = a_num + b_num;
    let u = a_num.max(b_num);
    let d = a_num.min(b_num);
    if let Some(&ans) = self.cache.get(&(d, u)) {
      return ans;
    }
    let ans = combinations(sum, d);
    self.cache.insert((d, u), ans);
    ans
  }
}

fn combinations(n: u128, k: u128) -> u128 {
  let mut n = n;
  let mut r: u128 = 1;
  for d in 1..=k {
    r *= n;
    n -= 1;
    r /= d;
  }
  r
}
