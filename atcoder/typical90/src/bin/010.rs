#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use std::ops::*;

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

trait IteratorExt: Iterator + Sized {
    fn vec(self) -> Vec<Self::Item> {
        self.collect()
    }
}

impl<T: Iterator> IteratorExt for T {}

trait ToChars {
    fn to_chars(&self) -> Vec<char>;
}

impl ToChars for i64 {
    fn to_chars(&self) -> Vec<char> {
        self.to_string().chars().collect()
    }
}

trait ToI64 {
    fn to_i64(&self) -> i64;
}

impl ToI64 for Vec<char> {
    fn to_i64(&self) -> i64 {
        self.iter().collect::<String>().parse().unwrap()
    }
}

pub trait Number {
    fn zero() -> Self;
    fn one() -> Self;
}
impl Number for i64 {
    fn zero() -> Self {
        0 as Self
    }
    fn one() -> Self {
        1 as Self
    }
}
impl Number for f64 {
    fn zero() -> Self {
        0 as Self
    }
    fn one() -> Self {
        1 as Self
    }
}

pub struct PrefixSum<T> {
    sum: Vec<T>,
    vec: Vec<T>,
    len: usize,
}
impl<T> From<Vec<T>> for PrefixSum<T>
where
    T: Number + Clone + Copy + Add<Output = T> + Sub<Output = T>,
{
    fn from(s: Vec<T>) -> Self {
        let len = s.len();
        let mut slf = Self {
            sum: vec![T::zero(); len + 1],
            vec: s.to_vec(),
            len,
        };
        slf.build();
        return slf;
    }
}
impl<T> PrefixSum<T>
where
    T: Number + Clone + Copy + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(len: usize) -> Self {
        Self {
            sum: vec![T::zero(); len + 1],
            vec: vec![T::zero(); len],
            len,
        }
    }
    pub fn add(&mut self, i: usize, element: T) {
        self.vec[i] = self.vec[i] + element;
    }
    pub fn build(&mut self) {
        for i in 0..self.len {
            self.sum[i + 1] = self.sum[i] + self.vec[i];
        }
    }
    pub fn query(&self, l: usize, r: usize) -> T {
        self.sum[r] - self.sum[l]
    }
}

fn main() {
    // let (r, w) = (std::io::stdin(), std::io::stdout());
    // let mut sc = IO::new(r.lock(), w.lock());

    input! {
        N: usize,
        cp: [(usize, i64); N],
        Q: usize,
        lr: [(usize, usize); Q],
    }

    let mut one = vec![0i64; 1e5 as usize + 10];
    let mut two = vec![0i64; 1e5 as usize + 10];
    for i in 0..N {
        if cp[i].0 == 1 {
            one[i] = cp[i].1;
        } else {
            two[i] = cp[i].1;
        }
    }
    let one = PrefixSum::from(one);
    let two = PrefixSum::from(two);
    for (l, r) in lr {
        println!("{} {}", one.query(l - 1, r), two.query(l - 1, r));
    }
}
