use std::io;
struct Edge {
    to: usize,
    nxt: usize,
    cap: i32,
    flow: i32,
}
struct MaxFlow {
    head: Vec<usize>,
    edge: Vec<Edge>,
}
impl MaxFlow {
    fn new(n: usize) -> Self {
        Self {
            head: vec![0x3f3f3f3f; n],
            edge: vec![],
        }
    }
    fn add_edge(&mut self, from: usize, to: usize, cap: i32) {
        let m = self.edge.len();
        self.edge.push(Edge { to, nxt: self.head[from], cap, flow: 0 });
        self.head[from] = m;
        self.edge.push(Edge { to: from, nxt: self.head[to], cap: 0, flow: 0 });
        self.head[to] = m + 1;
    }
}
fn main() {
    let (n, m, s, t) = match read_numbers_from_input().as_slice() {
        [n, m, s, t] => (*n, *m, *s, *t),
        _ => panic!("Extract numbers failed"),
    };
}
fn read_numbers_from_input() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to get input");
    input
        .split_whitespace()
        .filter_map(|word| word.parse::<i32>().ok())
        .collect()
}
