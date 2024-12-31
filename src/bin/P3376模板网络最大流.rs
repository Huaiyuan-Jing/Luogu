#[derive(Clone)]
struct Edge {
    to: usize,
    nxt: usize,
    cap: i32,
    flow: i32,
}
struct MaxFlow {
    s: usize,
    t: usize,
    head: Vec<usize>,
    edge: Vec<Edge>,
    maxflow: isize,
    depth: Vec<usize>,
}
impl MaxFlow {
    /// Creates a new `MaxFlow` instance with `n` nodes, source node `s`, and sink node `t`.
    pub fn new(n: usize, s: usize, t: usize) -> Self {
        Self {
            s,
            t,
            head: vec![0x3f3f3f3f; n],
            edge: vec![],
            maxflow: 0,
            depth: vec![0; n],
        }
    }
    
    /// Adds a directed edge from `from` to `to` with capacity `cap` to the flow network.
    ///
    /// # Arguments
    ///
    /// `from`: The node from which the edge originates.
    /// `to`: The node to which the edge points.
    /// `cap`: The capacity of the edge.
    pub fn add_edge(&mut self, from: usize, to: usize, cap: i32) {
        let m = self.edge.len();
        self.edge.push(Edge {
            to,
            nxt: self.head[from],
            cap,
            flow: 0,
        });
        self.head[from] = m;
        self.edge.push(Edge {
            to: from,
            nxt: self.head[to],
            cap: 0,
            flow: 0,
        });
        self.head[to] = m + 1;
    }
    /// Performs a BFS traversal of the residual graph to mark static nodes and find a shortest path from the source `s` to the sink `t`.
    ///
    /// Returns `true` if a path is found, and `false` otherwise.
    fn bfs(&mut self) -> bool {
        let mut q = std::collections::VecDeque::new();
        q.push_back(self.s);
        self.depth.fill(0x3f3f3f3f);
        self.depth[self.s] = 0;
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            let mut e = self.head[u];
            while e != 0x3f3f3f3f {
                let v = self.edge[e].to;
                if self.edge[e].flow < self.edge[e].cap && self.depth[v] > self.depth[u] + 1 {
                    self.depth[v] = self.depth[u] + 1;
                    q.push_back(v);
                }
                e = self.edge[e].nxt;
            }
        }
        self.depth[self.t] != 0x3f3f3f3f
    }
    /// Performs a DFS traversal of the residual graph to find a path from node `u` to the sink `t`.
    ///
    /// Returns the maximum flow that can be pushed along the path.
    ///
    /// # Arguments
    ///
    /// `u`: The starting node of the path.
    /// `flow`: The maximum flow to be pushed along the path.
    /// `edges`: The vector of edges in the residual graph.
    /// `cur`: The vector of current edges for each node in the residual graph.
    fn dfs(&self, u: usize, flow: i32, edges: &mut Vec<Edge>, cur: &mut Vec<usize>) -> i32 {
        if u == self.t || flow == 0 {
            return flow;
        }
        let mut res = 0;
        let mut e = cur[u];
        while e != 0x3f3f3f3f {
            let v = edges[e].to;
            if self.depth[v] == self.depth[u] + 1 {
                let f = self.dfs(
                    v,
                    std::cmp::min(flow - res, edges[e].cap - edges[e].flow),
                    edges,
                    cur,
                );
                edges[e].flow += f;
                edges[e ^ 1].flow -= f;
                res += f;
                if res == flow {
                    break;
                }
            }
            e = self.edge[e].nxt;
            cur[u] = e;
        }
        res
    }
    /// Computes the maximum flow in the flow network using the Dinic's algorithm.
    ///
    /// This function repeatedly calls `bfs` to find a shortest augmenting path
    /// and `dfs` to augment the flow along the path. The maximum flow is
    /// accumulated in `maxflow` field.
    pub fn dinic(&mut self) {
        while self.bfs() {
            let mut cur = self.head.clone();
            let mut edge = self.edge.clone();
            self.maxflow += self.dfs(self.s, std::i32::MAX, &mut edge, &mut cur) as isize;
            self.edge = edge;
        }
    }
}
fn main() {
    let (n, m, s, t) = match read_numbers_from_input().as_slice() {
        [n, m, s, t] => (*n, *m, *s, *t),
        _ => panic!("Extract numbers failed"),
    };
    let mut max_flow = MaxFlow::new(n as usize + 1, s as usize, t as usize);
    for _ in 0..m {
        let (u, v, c) = match read_numbers_from_input().as_slice() {
            [u, v, c] => (*u, *v, *c),
            _ => panic!("Extract numbers failed"),
        };
        max_flow.add_edge(u as usize, v as usize, c);
    }
    max_flow.dinic();
    println!("{}", max_flow.maxflow);
}
fn read_numbers_from_input() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to get input");
    input
        .split_whitespace()
        .filter_map(|word| word.parse::<i32>().ok())
        .collect()
}
