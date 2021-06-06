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

pub struct UnionFind {
    size: Vec<usize>,
    par: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut uf = Self {
            size: vec![0; n],
            par: vec![0; n],
        };
        for i in 0..n {
            uf.par[i] = i;
            uf.size[i] = 1;
        }
        uf
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        return self.find_root(x) == self.find_root(y);
    }
    pub fn find_root(&mut self, x: usize) -> usize {
        if x != self.par[x] {
            self.par[x] = self.find_root(self.par[x]);
        }
        return self.par[x];
    }
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let x = self.find_root(x);
        let y = self.find_root(y);
        if x == y {
            return false;
        }
        if self.size[x] > self.size[y] {
            self.par[y] = x;
            self.size[x] += self.size[y];
        } else {
            self.par[x] = y;
            self.size[y] += self.size[x];
        }
        return true;
    }
    pub fn size(&mut self, x: usize) -> usize {
        let x = self.find_root(x);
        return self.size[x];
    }
}

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let H: usize = sc.read();
    let W: usize = sc.read();
    let Q: usize = sc.read();

    let mut red = vec![vec![false; W]; H];
    let mut uf = UnionFind::new(W * H);
    let dx = [-1, 0, 1, 0];
    let dy = [0, 1, 0, -1];
    for _ in 0..Q {
        let t: usize = sc.read();
        if t == 1 {
            let (mut y, mut x): (usize, usize) = (sc.read(), sc.read());
            y -= 1;
            x -= 1;
            red[y][x] = true;
            for i in 0..4 {
                let ny = y as i64 + dy[i];
                let nx = x as i64 + dx[i];
                if ny >= 0 && nx >= 0 && nx < W as i64 && ny < H as i64 {
                    if red[ny as usize][nx as usize] {
                        let a = ny * W as i64 + nx;
                        let b = y * W + x;
                        uf.unite(a as usize, b);
                    }
                }
            }
        } else {
            let (mut y1, mut x1, mut y2, mut x2): (usize, usize, usize, usize) =
                (sc.read(), sc.read(), sc.read(), sc.read());
            y1 -= 1;
            x1 -= 1;
            y2 -= 1;
            x2 -= 1;
            let a = y1 * W + x1;
            let b = y2 * W + x2;
            let f = uf.is_same(a, b) && red[y1][x1] && red[y2][x2];
            println!("{}", if f { "Yes" } else { "No" });
        }
    }
}
