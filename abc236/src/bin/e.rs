fn main() {
  proconio::input! {
    n: usize,
    a: [u64; n],
  };
  #[derive(Copy, Clone, Debug, Default)]
  struct Tmp {
    pub count: u64,
    pub total: u64,
    pub av: f64,
    pub need: bool,
  };
  let mut dpav = vec![Tmp::default(); n];
  dpav[0] = Tmp {
    count: 1,
    total: a[0],
    av: a[0] as f64,
    need: false,
  };
  dpav[1] = {
    let av0 = dpav[0].av;
    let av1 = a[1] as f64;
    let av2 = (a[1] + a[0]) as f64 / 2.0;
    if av1 < av0 && av2 < av0 {
      Tmp {
        need: true,
        ..dpav[0]
      }
    } else if av0 < av1 && av2 < av1 {
      Tmp {
        count: 1,
        total: a[1],
        av: av1,
        need: false,
      }
    } else {
      Tmp {
        count: 2,
        total: a[1] + a[0],
        av: av2,
        need: false,
      }
    }
  };
  for i in 2..n {
    dpav[i] = {
      let av0 = dpav[i - 1].av;
      let total1 = a[i] + dpav[i - 2].total;
      let count1 = 1 + dpav[i - 2].count;
      let av1 = total1 as f64 / count1 as f64;
      let total2 = a[i] + dpav[i - 1].total;
      let count2 = 1 + dpav[i - 1].count;
      let av2 = total2 as f64 / count2 as f64;
      if av1 < av0 && av2 < av0 && !dpav[i - 1].need {
        Tmp {
          need: true,
          ..dpav[i - 1]
        }
      } else if av0 < av1 && av2 < av1 && !dpav[i - 2].need {
        Tmp {
          count: count1,
          total: total1,
          av: av1,
          need: false,
        }
      } else {
        Tmp {
          count: count2,
          total: total2,
          av: av2,
          need: false,
        }
      }
    };
  }
  println!("{:?}", &dpav);
  println!("{}", dpav[n - 1].av);
}
