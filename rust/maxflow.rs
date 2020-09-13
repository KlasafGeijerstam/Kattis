mod inp;
use inp::Inp;
use std::io::Result;

use std::collections::VecDeque;
use std::io::{Write, BufWriter};

const INF: i32 = 0x7fffffff;

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    cap: i32,
    rev: usize,
    is_back: bool
}

impl Edge {
    pub fn new(to: usize, cap: i32, rev: usize, is_back: bool) -> Self {
        Edge {
            to,
            cap,
            rev,
            is_back,
        }
    }
}

struct Dinic {
    graph: Vec<Vec<Edge>>,
    level: Vec<i32>,
    iter: Vec<usize>,
}

impl Dinic {
    pub fn new(nodes: usize) -> Self {
        Dinic {
            graph: vec![Vec::with_capacity(10); nodes],
            level: vec![0; nodes],
            iter: vec![0; nodes],
        }
    }
    
    pub fn add_edge(&mut self, from: usize, to: usize, cap: usize) {
        let rev = self.graph[to].len();
        self.graph[from].push(Edge::new(to, cap as i32, rev, false)); 
        let rev = self.graph[from].len() - 1;
        self.graph[to].push(Edge::new(from, 0, rev, true));
    }
    
    fn bfs(&mut self, s: usize) {
        for j in &mut self.level {
            *j = -1;
        }
        
        let mut que = VecDeque::new();
        self.level[s] = 0;
        que.push_back(s);

        while let Some(v) = que.pop_front() {
            for e in &self.graph[v] {
                if e.cap > 0 && self.level[e.to] < 0 {
                    self.level[e.to] = self.level[v] + 1;
                    que.push_back(e.to);
                }
            }
        }
    }

    fn dfs(&mut self, v: usize, t: usize, f: i32) -> i32 {
        if v == t {
            return f
        }

        let mut i = self.iter[v];
        while i < self.graph[v].len() {
            let e = self.graph[v][i];
            if e.cap > 0 && self.level[v] < self.level[e.to] {
                let d = self.dfs(e.to, t, i32::min(f, e.cap));
                if d > 0 {
                    self.graph[v][i].cap -= d;
                    self.graph[e.to][e.rev].cap += d;

                    return d;
                }
            }
            i += 1;
        }

        0
    }

    pub fn maximum_flow(&mut self, s: usize, t: usize) -> i32 {
        let mut flow = 0;

        loop {
            self.bfs(s);
            if self.level[t] < 0 {
                return flow
            }

            for i in &mut self.iter {
                *i = 0;
            }

            loop {
                let f = self.dfs(s, t, INF);
                if f > 0 {
                    flow += f;
                } else {
                    break
                }
            }
        }
    }

    pub fn flow_graph(&mut self, t: usize) -> Vec<(usize, usize, i32)> {
        let mut que = VecDeque::new();
        let mut visited = vec![false; self.graph.len()];
        let mut output = Vec::new();
        que.push_back(t);
        while let Some(v) = que.pop_front() {
            if visited[v] {
                continue
            }
            visited[v] = true;
            for e in &self.graph[v] {
                if e.is_back && e.cap > 0 {
                    output.push((e.to, v, e.cap));
                }

                if !visited[e.to]  {
                    que.push_back(e.to);
                }
            }
        }

        output

    }
}

fn main() -> Result<()> {
    let mut inp = Inp::new();
    let (n, m, s, t) = inp.next_quad();
    let mut d = Dinic::new(n);
    for _ in 0..m {
        let (u, v, c) = inp.next_triple();
        d.add_edge(u, v, c);
    }

    let f = d.maximum_flow(s, t);
    let o = d.flow_graph(t);
    let stdout = std::io::stdout();
    let slock = stdout.lock();
    let mut writer = BufWriter::new(slock);
    writeln!(writer, "{} {} {}", n, f, o.len())?;
    for (u, v, c) in o {
        writeln!(writer, "{} {} {}", u, v, c)?;
    }
    Ok(())
)
