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

macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_min = min!($($cmps),+);
        if $base > cmp_min {
            $base = cmp_min;
            true
        } else {
            false
        }
    }};
}

macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_max = max!($($cmps),+);
        if $base < cmp_max {
            $base = cmp_max;
            true
        } else {
            false
        }
    }};
}

macro_rules! min {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::min($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::min($a, min!($($rest),+))
    }};
}

macro_rules! max {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::max($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::max($a, max!($($rest),+))
    }};
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

use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

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

impl<T> Matrix<T> {
    pub fn new(v: Vec<Vec<T>>) -> Self {
        Self {
            h: v.len(),
            w: v[0].len(),
            v,
        }
    }

    pub fn identity(n: usize) -> Self
    where
        T: Number + Clone,
    {
        let mut v = vec![vec![T::zero(); n]; n];
        for i in 0..n {
            v[i][i] = T::one();
        }

        Matrix::<T>::new(v)
    }

    pub fn mul_vec(&mut self, rhs: Vec<T>) -> Vec<T>
    where
        T: Mul<Output = T> + AddAssign + Number + Clone + Copy,
    {
        let mut v = vec![T::zero(); rhs.len()];
        for i in 0..self.h {
            for j in 0..self.w {
                v[i] += self.v[i][j] * rhs[j];
            }
        }
        v
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    h: usize,
    w: usize,
    v: Vec<Vec<T>>,
}

impl<T> AddAssign for Matrix<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..self.h {
            for j in 0..self.w {
                self.v[i][j] += rhs.v[i][j];
            }
        }
    }
}

impl<T> SubAssign for Matrix<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..self.h {
            for j in 0..self.w {
                self.v[i][j] -= rhs.v[i][j];
            }
        }
    }
}

impl<T> MulAssign for Matrix<T>
where
    T: AddAssign + Mul<Output = T> + Number + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        let mut v = vec![vec![T::zero(); rhs.w]; self.h];
        for i in 0..self.h {
            for j in 0..rhs.w {
                for k in 0..rhs.h {
                    v[i][j] += self.v[i][k] * rhs.v[k][j];
                }
            }
        }
        *self = Matrix::new(v);
    }
}

impl<T> Add for Matrix<T>
where
    T: AddAssign + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res += rhs;
        res
    }
}

impl<T> Mul for Matrix<T>
where
    T: AddAssign + Mul<Output = T> + Number + Copy,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res *= rhs;
        res
    }
}

impl<T> Sub for Matrix<T>
where
    T: SubAssign + Copy,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res -= rhs;
        res
    }
}

fn main() {
    // let (r, w) = (std::io::stdin(), std::io::stdout());
    // let mut sc = IO::new(r.lock(), w.lock());
    input! {
        N: usize,
        p0: (f64, f64),
        p1: (f64, f64),
    }
    let theta = 2. * PI / N as f64;
    let mat1 = Matrix::new(vec![
        vec![theta.cos(), -theta.sin(), 0.],
        vec![theta.sin(), theta.cos(), 0.],
        vec![0., 0., 1.],
    ]);
    let o = ((p0.0 + p1.0) / 2., (p0.1 + p1.1) / 2.);
    let mat2 = Matrix::new(vec![vec![1., 0., o.0], vec![0., 1., o.1], vec![0., 0., 1.]]);
    let mat3 = Matrix::new(vec![
        vec![1., 0., -o.0],
        vec![0., 1., -o.1],
        vec![0., 0., 1.],
    ]);
    let vec = vec![p0.0, p0.1, 1.];
    let mut mat = mat2 * mat1 * mat3;
    let ans = mat.mul_vec(vec);
    println!("{} {}", ans[0], ans[1]);
}
