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

fn dfs(graph: &[Vec<usize>], u: usize, par: Option<usize>) -> (usize, usize) {
    let mut res = (0, u);
    for e in &graph[u] {
        if par == Some(*e) {
            continue;
        }
        let mut next = dfs(graph, *e, Some(u));
        next.0 += 1;
        res = std::cmp::max(res, next)
    }
    res
}

fn main() {
    // let (r, w) = (std::io::stdin(), std::io::stdout());
    // let mut sc = IO::new(r.lock(), w.lock());
    input! {
        N: usize,
        AB: [(usize1, usize1); N-1],
    }

    let mut graph = vec![vec![]; N];
    for (a, b) in AB {
        graph[a].push(b);
        graph[b].push(a);
    }
    let mut res = dfs(&graph, 0, None);
    res = dfs(&graph, res.1, None);
    println!("{}", res.0 + 1)
}
