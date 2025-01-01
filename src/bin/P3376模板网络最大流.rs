mod maxflow {
    #[derive(Clone)]
    struct Edge {
        to: usize,
        nxt: usize,
        cap: i32,
        flow: i32,
    }
    pub struct Graph {
        n: usize,
        s: usize,
        t: usize,
        head: Vec<usize>,
        edge: Vec<Edge>,
        pub maxflow: isize,
        depth: Vec<usize>,
        gap: Vec<usize>,
    }
    impl Graph {
        /// Creates a new `MaxFlow` instance with `n` nodes, source node `s`, and sink node `t`.
        pub fn new(n: usize, s: usize, t: usize) -> Self {
            Self {
                n,
                s,
                t,
                head: vec![0x3f3f3f3f; n],
                edge: vec![],
                maxflow: 0,
                depth: vec![0x3f3f3f3f; n],
                gap: vec![0; n * 2],
            }
        }
    }
    /// Adds a directed edge from `from` to `to` with capacity `cap` to the flow network.
    ///
    /// # Arguments
    ///
    /// `from`: The node from which the edge originates.
    /// `to`: The node to which the edge points.
    /// `cap`: The capacity of the edge.
    pub fn add_edge(g: &mut Graph, from: usize, to: usize, cap: i32) {
        let m = g.edge.len();
        g.edge.push(Edge {
            to,
            nxt: g.head[from],
            cap,
            flow: 0,
        });
        g.head[from] = m;
        g.edge.push(Edge {
            to: from,
            nxt: g.head[to],
            cap: 0,
            flow: 0,
        });
        g.head[to] = m + 1;
    }
    /// Performs a BFS traversal of the residual graph to mark static nodes and find a shortest path from the source `s` to the sink `t`.
    ///
    /// Returns `true` if a path is found, and `false` otherwise.
    fn generate_depth(g: &mut Graph) {
        g.depth = vec![0x3f3f3f3f; g.n];
        let mut q = std::collections::VecDeque::new();
        q.push_back(g.t);
        g.depth[g.t] = 0;
        g.gap[0] = 1;
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            let mut e = g.head[u];
            while e != 0x3f3f3f3f {
                let v = g.edge[e].to;
                if g.depth[v] != 0x3f3f3f3f {
                    e = g.edge[e].nxt;
                    continue;
                }
                g.depth[v] = g.depth[u] + 1;
                g.gap[g.depth[v]] += 1;
                q.push_back(v);
                e = g.edge[e].nxt;
            }
        }
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
    fn dfs(g: &mut Graph, u: usize, flow: i32, cur: &mut Vec<usize>) -> i32 {
        if u == g.t || flow == 0 {
            return flow;
        }
        let mut used = 0;
        let mut e = cur[u];
        while e != 0x3f3f3f3f {
            let v = g.edge[e].to;
            if g.depth[v] == g.depth[u] - 1 {
                let f = dfs(
                    g,
                    v,
                    std::cmp::min(flow - used, g.edge[e].cap - g.edge[e].flow),
                    cur,
                );
                g.edge[e].flow += f;
                g.edge[e ^ 1].flow -= f;
                used += f;
                if used == flow {
                    return used;
                }
            }
            e = g.edge[e].nxt;
            cur[u] = e;
        }
        g.gap[g.depth[u]] -= 1;
        if g.gap[g.depth[u]] == 0 {
            g.depth[g.s] = g.n + 1;
        }
        g.depth[u] += 1;
        g.gap[g.depth[u]] += 1;
        used
    }
    /// Computes the maximum flow in the flow network using the Dinic's algorithm.
    ///
    /// This function repeatedly calls `bfs` to find a shortest augmenting path
    /// and `dfs` to augment the flow along the path. The maximum flow is
    /// accumulated in `maxflow` field.
    pub fn isap(g: &mut Graph) {
        generate_depth(g);
        while g.depth[g.s] < g.n {
            let mut cur = g.head.clone();
            g.maxflow += dfs(g, g.s, std::i32::MAX, &mut cur) as isize;
        }
    }
}
fn main() {
    let (n, m, s, t) = match read_numbers_from_input().as_slice() {
        [n, m, s, t] => (*n, *m, *s, *t),
        _ => panic!("Extract numbers failed"),
    };
    let mut g = maxflow::Graph::new(n as usize + 1, s as usize, t as usize);
    for _ in 0..m {
        let (u, v, c) = match read_numbers_from_input().as_slice() {
            [u, v, c] => (*u, *v, *c),
            _ => panic!("Extract numbers failed"),
        };
        maxflow::add_edge(&mut g,u as usize, v as usize, c);
    }
    maxflow::isap(&mut g);
    println!("{}", g.maxflow);
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
