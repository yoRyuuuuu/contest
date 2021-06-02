#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use std::fmt::Binary;

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

pub struct BinarySearch<F> {
    f: F,
    ok: i64,
    ng: i64,
}
impl<F> BinarySearch<F>
where
    F: FnMut(i64) -> bool,
{
    pub fn new(f: F, ok: i64, ng: i64) -> Self {
        Self { f, ok, ng }
    }
    pub fn search(&mut self) -> i64 {
        let mut ok = self.ok;
        let mut ng = self.ng;
        while (ok - ng).abs() > 1 {
            let mid = (ng + ok) / 2;
            if (self.f)(mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    }
}

pub struct Ruisekiwa2D {
    sum: Vec<Vec<i64>>,
    vec: Vec<Vec<i64>>,
    h: usize,
    w: usize,
}

impl Ruisekiwa2D {
    pub fn new(h: usize, w: usize) -> Self {
        Self {
            sum: vec![vec![0i64; w + 1]; h + 1],
            vec: vec![vec![0i64; w]; h],
            h,
            w,
        }
    }

    pub fn add(&mut self, x: usize, y: usize, element: i64) {
        self.vec[y][x] += element;
    }

    pub fn build(&mut self) {
        for y in 0..self.h {
            for x in 0..self.w {
                self.sum[y + 1][x + 1] =
                    self.sum[y + 1][x] + self.sum[y][x + 1] - self.sum[y][x] + self.vec[y][x];
            }
        }
    }

    pub fn query(&self, x1: usize, x2: usize, y1: usize, y2: usize) -> i64 {
        self.sum[y2][x2] - self.sum[y1][x2] - self.sum[y2][x1] + self.sum[y1][x1]
    }
}

fn main() {
    // let (r, w) = (std::io::stdin(), std::io::stdout());
    // let mut sc = IO::new(r.lock(), w.lock());
    input! {
        N: usize,
        K: usize,
        A: [[i64; N]; N],
    }

    let f = |x: i64| -> bool {
        let mut rui = Ruisekiwa2D::new(N, N);
        for i in 0..N {
            for j in 0..N {
                if x <= A[i][j] {
                    rui.add(i, j, 1);
                }
            }
        }
        rui.build();
        for i in 0..N - K + 1 {
            for j in 0..N - K + 1 {
                let sum = rui.query(i, i + K, j, j + K) as usize;
                if sum < K * K / 2 + 1 {
                    return false;
                }
            }
        }
        return true;
    };

    let mut bs = BinarySearch::new(f, 0, 1e10 as i64);
    println!("{}", bs.search());
}
