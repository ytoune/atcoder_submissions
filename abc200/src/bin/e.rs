fn main() {
  proconio::input! { size: u128, target: u128 };
  struct Solve {
    pub params: [u128; 3],
  }

  let mut idx = 1;
  let mut add1 = 3;
  let mut add2 = 3;
  let mut add3 = true;
  let mut start = 1;
  loop {
    let next = idx + add1;
    // println!("{:?}", (next, add1, add2, add3));
    if target <= next {
      break;
    }
    start += 1;
    idx = next;
    if add3 && size + 1 == add2 {
      add3 = false;
    } else if add3 {
      add1 += add2;
      add2 += 1;
    } else if add1 >= add2 {
      add1 -= add2;
      add2 -= 1;
    } else {
      // println!("{:?}", (next, add1, add2, add3));
      // panic!("?");
      break;
    }
  }

  let mut st = Solve { params: [1; 3] };
  st.params[0] = start;
  impl Solve {
    fn next(&mut self) {
      if 1 == self.params[1] && 1 == self.params[2] {
        self.params[2] = self.params[0] + 1;
        self.params[0] = 1;
        return;
      }
      if 1 == self.params[2] {
        self.params[0] += 1;
        self.params[2] = self.params[1] - 1;
        self.params[1] = 1;
        return;
      }
      self.params[2] -= 1;
      self.params[1] += 1;
    }
  }
  while idx < target {
    st.next();
    if st.params.iter().any(|&p| size < p) {
      continue;
    }
    idx += 1;
  }
  println!("{} {} {}", st.params[0], st.params[1], st.params[2]);
}
