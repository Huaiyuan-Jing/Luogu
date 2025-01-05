mod maxflow {
    #[derive(Clone)]
    struct Edge {
        to: usize,
        nxt: usize,
        cap: i32,
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
        remain: Vec<i32>,
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
                remain: vec![0; n],
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
        });
        g.head[from] = m;
        g.edge.push(Edge {
            to: from,
            nxt: g.head[to],
            cap: 0,
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
                if g.depth[v] > g.depth[u] + 1 || g.edge[e ^ 1].cap <= 0 {
                    e = g.edge[e].nxt;
                    continue;
                }
                g.depth[v] = g.depth[u] + 1;
                q.push_back(v);
                e = g.edge[e].nxt;
            }
        }
    }
    /// Computes the maximum flow in the flow network using the Dinic's algorithm.
    ///
    /// This function repeatedly calls `bfs` to find a shortest augmenting path
    /// and `dfs` to augment the flow along the path. The maximum flow is
    /// accumulated in `maxflow` field.
        pub fn hlpp(g: &mut Graph) {
        generate_depth(g);
        if g.depth[g.s] >= g.n {
            g.maxflow = 0;
            return;
        }

        let mut excess = vec![0; g.n];
        let mut active = vec![false; g.n];
        let mut queue = std::collections::VecDeque::new();

        // Initialize excess flows and push/relabel queues
        let mut enqueue = |u: usize| {
            if !active[u] && u != g.s && u != g.t {
                active[u] = true;
                queue.push_back(u);
            }
        };

        // Initialize preflow
        let mut e = g.head[g.s];
        while e != 0x3f3f3f3f {
            let edge = &mut g.edge[e];
            let v = edge.to;
            if edge.cap > 0 {
                let delta = edge.cap;
                edge.cap -= delta;
                g.edge[e ^ 1].cap += delta;
                excess[v] += delta;
                enqueue(v);
            }
            e = edge.nxt;
        }

        // Push operation
        let mut push = |u: usize, e: usize| {
            let edge = &mut g.edge[e];
            let v = edge.to;
            let delta = std::cmp::min(excess[u], edge.cap);
            if delta > 0 && g.depth[u] == g.depth[v] + 1 {
                edge.cap -= delta;
                g.edge[e ^ 1].cap += delta;
                excess[u] -= delta;
                excess[v] += delta;
                enqueue(v);
            }
        };

        // Relabel operation
        let mut relabel = |u: usize| {
            let mut min_depth = usize::MAX;
            let mut e = g.head[u];
            while e != 0x3f3f3f3f {
                let edge = &g.edge[e];
                if edge.cap > 0 {
                    min_depth = std::cmp::min(min_depth, g.depth[edge.to]);
                }
                e = edge.nxt;
            }
            if min_depth != usize::MAX {
                g.gap[g.depth[u]] -= 1;
                g.depth[u] = min_depth + 1;
                g.gap[g.depth[u]] += 1;
                enqueue(u);
            }
        };

        // Process active nodes
        while let Some(u) = queue.pop_front() {
            active[u] = false;
            let mut e = g.head[u];
            while excess[u] > 0 && e != 0x3f3f3f3f {
                push(u, e);
                e = g.edge[e].nxt;
            }
            if excess[u] > 0 {
                relabel(u);
            }
        }

        // Compute max flow
        g.maxflow = excess[g.t] as isize;
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
        maxflow::add_edge(&mut g, u as usize, v as usize, c);
    }
    maxflow::hlpp(&mut g);
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
