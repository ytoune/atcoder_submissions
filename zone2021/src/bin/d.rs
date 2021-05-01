fn main() {
  use proconio::marker::Chars;
  proconio::input! { s: Chars };
  use std::collections::*;
  let mut ans = VecDeque::<char>::new();
  let mut is_r = false;
  for c in s {
    match c {
      'R' => {
        is_r = !is_r;
      }
      c => {
        let top = if is_r { ans.front() } else { ans.back() };
        if top == Some(&c) {
          if is_r {
            ans.pop_front();
          } else {
            ans.pop_back();
          }
        } else if is_r {
          ans.push_front(c);
        } else {
          ans.push_back(c);
        }
      }
    }
  }
  let ans: String = if is_r {
    ans.iter().rev().collect()
  } else {
    ans.iter().collect()
  };
  println!("{}", ans);
}
