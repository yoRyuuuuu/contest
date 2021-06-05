#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> Self {
        Self(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: ToString>(&mut self, s: S) {
        use std::io::Write;
        self.1.write_all(s.to_string().as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Total<T>(pub T);

impl<T: PartialEq> Eq for Total<T> {}

impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

use std::ops::*;
#[derive(Clone, Copy, Debug)]
pub struct Complex {
    pub x: f64,
    pub y: f64,
}
impl Complex {
    pub fn new(x: f64, y: f64) -> Self {
        Complex { x, y }
    }
    pub fn abs(&self) -> f64 {
        (self.x * self.x + self.y + self.y).sqrt()
    }
    pub fn arg(&self) -> f64 {
        self.y.atan2(self.x)
    }
    pub fn polar(r: f64, theta: f64) -> Self {
        Complex::new(r * theta.cos(), r * theta.sin())
    }
    pub fn con(&self) -> Self {
        Complex::new(self.x, -self.y)
    }
    pub fn angle(&self) -> f64 {
        if self.y >= 0.0 {
            let tmp = self.x / (self.x * self.x + self.y * self.y).sqrt();
            let angle = tmp.acos() * 180.0 / std::f64::consts::PI;
            return angle;
        } else {
            let tmp = self.x / (self.x * self.x + self.y * self.y).sqrt();
            let angle = tmp.acos() * 180.0 / std::f64::consts::PI;
            return 360. - angle;
        }
    }
}
impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Complex::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Complex::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Complex::new(
            self.x * rhs.x - self.y * rhs.y,
            self.x * rhs.y + self.y * rhs.x,
        )
    }
}
impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let a = self * self.con();
        let b = rhs.x * rhs.x + rhs.y + rhs.y;
        Complex::new(a.x / b, a.y / b)
    }
}
impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl DivAssign for Complex {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

pub trait BinarySearchExt<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}
impl<T: Ord> BinarySearchExt<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        use std::cmp::Ordering;
        let mut ng = -1i64;
        let mut ok = self.len() as i64;
        while ok - ng > 1 {
            let mid = (ok + ng) as usize / 2;
            match self[mid].cmp(x) {
                Ordering::Greater | Ordering::Equal => {
                    ok = mid as i64;
                }
                _ => ng = mid as i64,
            }
        }
        ok as usize
    }
    fn upper_bound(&self, x: &T) -> usize {
        use std::cmp::Ordering;
        let mut ng = -1i64;
        let mut ok = self.len() as i64;
        while ok - ng > 1 {
            let mid = (ok + ng) as usize / 2;
            match self[mid].cmp(x) {
                Ordering::Greater => {
                    ok = mid as i64;
                }
                _ => {
                    ng = mid as i64;
                }
            }
        }
        ok as usize
    }
}

fn main() {
    // let (r, w) = (std::io::stdin(), std::io::stdout());
    // let mut sc = IO::new(r.lock(), w.lock());

    input! {
        N: usize,
        xy: [(i64, i64); N],
    }

    let cs = xy
        .into_iter()
        .map(|(x, y)| Complex::new(x as f64, y as f64))
        .collect::<Vec<_>>();

    let get_angle = |a: f64, b: f64| {
        let res = (a - b).abs();
        if res >= 180. {
            return 360. - res;
        } else {
            return res;
        }
    };

    let solve = |pos: usize| {
        let mut vec = vec![];
        for i in 0..N {
            if i == pos {
                continue;
            }
            let a = cs[i] - cs[pos];
            let angle = a.angle();
            vec.push(Total(angle));
        }
        vec.sort();

        let mut res = 0.;
        for i in 0..vec.len() {
            let mut target = vec[i].0 + 180.;
            if target >= 360. {
                target -= 360.;
            }
            let pos1 = vec.lower_bound(&Total(target));

            let idx1 = pos1 % vec.len();
            let idx2 = (pos1 + vec.len() - 1) % vec.len();
            let idx3 = (pos1 + vec.len() + 1) % vec.len();
            let cand1 = get_angle(vec[i].0, vec[idx1].0);
            let cand2 = get_angle(vec[i].0, vec[idx2].0);
            let cand3 = get_angle(vec[i].0, vec[idx3].0);
            res = f64::max(res, cand1);
            res = f64::max(res, cand2);
            res = f64::max(res, cand3);
        }

        return res;
    };

    let mut ans = 0.;
    for i in 0..N {
        let res = solve(i);
        ans = f64::max(ans, res);
    }

    println!("{}", ans);
}
