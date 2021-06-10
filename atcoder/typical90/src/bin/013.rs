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

use std::cmp::Ordering;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Edge {
    to: usize,
    cost: i64,
}

impl Edge {
    pub fn new(to: usize, cost: i64) -> Self {
        Self { to, cost }
    }
}

pub fn dijkstra(graph: &Vec<Vec<Edge>>, start: usize) -> Vec<i64> {
    use std::collections::BinaryHeap;
    let mut dist: Vec<_> = (0..graph.len()).map(|_| std::i64::MAX).collect();
    let mut heap = BinaryHeap::new();
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        cur_node: start,
    });

    while let Some(State { cost, cur_node }) = heap.pop() {
        if cost > dist[cur_node] {
            continue;
        }

        for edge in &graph[cur_node] {
            let next = State {
                cost: cost + edge.cost,
                cur_node: edge.to,
            };

            if next.cost < dist[next.cur_node] {
                heap.push(next);
                dist[next.cur_node] = next.cost;
            }
        }
    }

    return dist;
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct State {
    cost: i64,
    cur_node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.cur_node.cmp(&other.cur_node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    // let (r, w) = (std::io::stdin(), std::io::stdout());
    // let mut sc = IO::new(r.lock(), w.lock());

    input! {
        N: usize,
        M: usize,
        abc: [(usize1, usize1, i64); M],
    }

    let mut graph = vec![vec![]; N];
    for (a, b, c) in abc {
        graph[a].push(Edge::new(b, c));
        graph[b].push(Edge::new(a, c));
    }

    let d1 = dijkstra(&graph, 0);
    let d2 = dijkstra(&graph, N - 1);
    for k in 0..N {
        println!("{}", d1[k] + d2[k]);
    }
}
